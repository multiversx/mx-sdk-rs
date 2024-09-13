use colored::Colorize;
use std::{
    collections::{HashMap, HashSet},
    fs,
};
use wasmparser::{
    BinaryReaderError, DataSectionReader, ExportSectionReader, FunctionBody, ImportSectionReader,
    Operator, Parser, Payload,
};

use crate::ei::EIVersion;

use super::report_creator::{ReportCreator, WITHOUT_MESSAGE, WITH_MESSAGE};

type CallGraph = HashMap<usize, HashSet<usize>>;

const PANIC_WITH_MESSAGE: &[u8; 16] = b"panic occurred: ";
const PANIC_WITHOUT_MESSAGE: &[u8; 14] = b"panic occurred";
const ERROR_FAIL_ALLOCATOR: &[u8; 27] = b"memory allocation forbidden";
const MEMORY_GROW_OPCODE: u8 = 0x40;
const WRITE_OP: [&str; 1] = ["mBufferStorageStore"];

#[derive(Default)]
pub struct WasmInfo {
    pub imports: Vec<String>,
    pub ei_check: bool,
    pub memory_grow_flag: bool,
    pub report: ReportCreator,
    pub call_graph: CallGraph,
}

impl WasmInfo {
    pub fn extract_wasm_info(
        output_wasm_path: &str,
        extract_imports_enabled: bool,
        check_ei: &Option<EIVersion>,
        view_endpoints: HashMap<&str, usize>,
    ) -> Result<WasmInfo, BinaryReaderError> {
        let wasm_data = fs::read(output_wasm_path)
            .expect("error occured while extracting information from .wasm: file not found");

        populate_wasm_info(
            output_wasm_path.to_string(),
            wasm_data,
            extract_imports_enabled,
            check_ei,
            view_endpoints,
        )
    }
}

fn populate_wasm_info(
    path: String,
    wasm_data: Vec<u8>,
    extract_imports_enabled: bool,
    check_ei: &Option<EIVersion>,
    mut view_endpoints: HashMap<&str, usize>,
) -> Result<WasmInfo, BinaryReaderError> {
    let mut wasm_info = WasmInfo::default();
    let mut write_functions: HashSet<usize> = HashSet::new();

    let parser = Parser::new(0);
    for payload in parser.parse_all(&wasm_data) {
        match payload? {
            Payload::ImportSection(import_section) => {
                write_functions =
                    process_imports(import_section, extract_imports_enabled, &mut wasm_info);
                wasm_info.ei_check = is_ei_valid(&wasm_info.imports, check_ei);
            },
            Payload::DataSection(data_section) => {
                wasm_info.report.has_allocator = is_fail_allocator_triggered(data_section.clone());
                if is_panic_with_message_triggered(data_section.clone()) {
                    wasm_info.report.has_panic = WITH_MESSAGE.to_owned();
                } else if is_panic_without_message_triggered(data_section) {
                    wasm_info.report.has_panic = WITHOUT_MESSAGE.to_owned();
                }
            },
            Payload::CodeSectionEntry(code_section) => {
                wasm_info.memory_grow_flag = is_mem_grow(&code_section);
                create_call_graph(code_section, &mut wasm_info.call_graph);
            },
            Payload::ExportSection(export_section) => {
                parse_export_section(export_section, &mut view_endpoints);
            },
            _ => (),
        }
    }

    detect_write_operations_in_views(&view_endpoints, &wasm_info.call_graph, &mut write_functions);

    let report = ReportCreator {
        path,
        has_allocator: wasm_info.report.has_allocator,
        has_panic: wasm_info.report.has_panic,
    };

    Ok(WasmInfo {
        imports: wasm_info.imports,
        ei_check: wasm_info.ei_check,
        memory_grow_flag: wasm_info.memory_grow_flag,
        call_graph: wasm_info.call_graph,
        report,
    })
}

fn detect_write_operations_in_views(
    views_data: &HashMap<&str, usize>,
    call_graph: &CallGraph,
    write_functions: &mut HashSet<usize>,
) {
    let mut visited: HashSet<usize> = HashSet::new();
    for index in views_data.values() {
        mark_write(*index, call_graph, write_functions, &mut visited);
    }

    for (name, index) in views_data {
        if write_functions.contains(index) {
            println!(
                "{} {}",
                "Write storage operation in VIEW endpoint:"
                    .to_string()
                    .red()
                    .bold(),
                name.red().bold()
            );
        }
    }
}

fn parse_export_section(
    export_section: ExportSectionReader,
    view_endpoints: &mut HashMap<&str, usize>,
) {
    for export in export_section {
        let export = export.expect("Failed to read export section");
        if let wasmparser::ExternalKind::Func = export.kind {
            if let Some(endpoint_index) = view_endpoints.get_mut(export.name) {
                *endpoint_index = export.index.try_into().unwrap();
            }
        }
    }
}

fn mark_write(
    func: usize,
    call_graph: &CallGraph,
    write_functions: &mut HashSet<usize>,
    visited: &mut HashSet<usize>,
) {
    // Return early to prevent cycles.
    if visited.contains(&func) {
        return;
    }

    visited.insert(func);

    if let Some(callees) = call_graph.get(&func) {
        for &callee in callees {
            if write_functions.contains(&callee) {
                write_functions.insert(func);
            } else {
                mark_write(callee, call_graph, write_functions, visited);
                if write_functions.contains(&callee) {
                    write_functions.insert(func);
                }
            }
        }
    }
}

fn create_call_graph(body: FunctionBody, call_graph: &mut CallGraph) {
    let mut instructions_reader = body
        .get_operators_reader()
        .expect("Failed to get operators reader");

    let mut call_functions = HashSet::new();
    while let Ok(op) = instructions_reader.read() {
        if let Operator::Call { function_index } = op {
            let function_usize: usize = function_index.try_into().unwrap();
            call_functions.insert(function_usize);
        }
    }

    call_graph.insert(call_graph.len(), call_functions);
}

fn is_fail_allocator_triggered(data_section: DataSectionReader) -> bool {
    for data_fragment in data_section.into_iter().flatten() {
        if data_fragment
            .data
            .windows(ERROR_FAIL_ALLOCATOR.len())
            .any(|data| data == ERROR_FAIL_ALLOCATOR)
        {
            println!(
                "{}",
                "FailAllocator used while memory allocation is accessible in code. Contract may fail unexpectedly when memory allocation is attempted"
                    .to_string()
                    .red()
                    .bold()
            );
            return true;
        }
    }

    false
}

fn is_panic_with_message_triggered(data_section: DataSectionReader) -> bool {
    for data_fragment in data_section.into_iter().flatten() {
        if data_fragment
            .data
            .windows(PANIC_WITH_MESSAGE.len())
            .any(|data| data == PANIC_WITH_MESSAGE)
        {
            return true;
        }
    }

    false
}

fn is_panic_without_message_triggered(data_section: DataSectionReader) -> bool {
    for data_fragment in data_section.into_iter().flatten() {
        if data_fragment
            .data
            .windows(PANIC_WITHOUT_MESSAGE.len())
            .any(|data| data == PANIC_WITHOUT_MESSAGE)
        {
            return true;
        }
    }

    false
}

pub fn process_imports(
    import_section: ImportSectionReader,
    import_extraction_enabled: bool,
    wasm_info: &mut WasmInfo,
) -> HashSet<usize> {
    let mut write_functions = HashSet::new();
    for (index, import) in import_section.into_iter().flatten().enumerate() {
        if import_extraction_enabled {
            wasm_info.imports.push(import.name.to_string());
        }
        wasm_info.call_graph.insert(index, HashSet::new());
        if WRITE_OP.contains(&import.name) {
            write_functions.insert(index);
        }
    }

    wasm_info.imports.sort();

    write_functions
}

fn is_ei_valid(imports: &[String], check_ei: &Option<EIVersion>) -> bool {
    if let Some(ei) = check_ei {
        let mut num_errors = 0;
        for import in imports {
            if !ei.contains_vm_hook(import.as_str()) {
                num_errors += 1;
            }
        }

        if num_errors == 0 {
            return true;
        }
    }

    false
}

fn is_mem_grow(code_section: &FunctionBody) -> bool {
    let mut code = code_section.get_binary_reader();
    while code.bytes_remaining() > 0 {
        if code.read_u8().unwrap() == MEMORY_GROW_OPCODE {
            return true;
        }
    }
    false
}
