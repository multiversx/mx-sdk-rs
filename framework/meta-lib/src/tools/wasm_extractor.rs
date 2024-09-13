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

const PANIC_WITH_MESSAGE: &[u8; 16] = b"panic occurred: ";
const PANIC_WITHOUT_MESSAGE: &[u8; 14] = b"panic occurred";
const ERROR_FAIL_ALLOCATOR: &[u8; 27] = b"memory allocation forbidden";
const MEMORY_GROW_OPCODE: u8 = 0x40;
const WRITE_OP: [&str; 1] = ["mBufferStorageStore"];

pub struct WasmInfo {
    pub imports: Vec<String>,
    pub ei_check: bool,
    pub memory_grow_flag: bool,
    pub has_format: bool,
    pub report: ReportCreator,
}

impl WasmInfo {
    pub fn extract_wasm_info(
        output_wasm_path: &str,
        extract_imports_enabled: bool,
        check_ei: &Option<EIVersion>,
        view_endpoints: Vec<String>,
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
    view_endpoints: Vec<String>,
) -> Result<WasmInfo, BinaryReaderError> {
    let mut imports = Vec::new();
    let mut allocator_trigger = false;
    let mut ei_check = false;
    let mut memory_grow_flag = false;
    let mut has_panic = "none";
    let mut call_graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut views_data: HashMap<usize, String> = HashMap::new();
    let mut write_functions: Vec<usize> = Vec::new();

    let mut parser = Parser::new(0);
    for payload in parser.parse_all(&wasm_data) {
        match payload? {
            Payload::ImportSection(import_section) => {
                imports = extract_imports(
                    import_section,
                    extract_imports_enabled,
                    &mut call_graph,
                    &mut write_functions,
                );
                ei_check = is_ei_valid(&imports, check_ei);
            },
            Payload::DataSection(data_section) => {
                allocator_trigger = is_fail_allocator_triggered(data_section.clone());
                if is_panic_with_message_triggered(data_section.clone()) {
                    has_panic = WITH_MESSAGE;
                } else if is_panic_without_message_triggered(data_section) {
                    has_panic = WITHOUT_MESSAGE;
                }
            },
            Payload::CodeSectionEntry(code_section) => {
                memory_grow_flag = is_mem_grow(&code_section);
            },
            Payload::ExportSection(export_section) => {
                views_data = parse_export_section(export_section, &view_endpoints);
            },
            _ => (),
        }
    }

    parser = Parser::new(0);
    for payload in parser.parse_all(&wasm_data) {
        if let Payload::CodeSectionEntry(body) = payload? {
            create_call_graph(body, &mut call_graph);
        }
    }
    let mut visited: HashSet<usize> = HashSet::new();
    for key in views_data.keys() {
        mark_write(*key, &call_graph, &mut write_functions, &mut visited);
    }

    for (index, name) in views_data {
        if write_functions.contains(&index) {
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

    let report = ReportCreator {
        path,
        has_allocator: allocator_trigger,
        has_panic: has_panic.to_string(),
    };

    Ok(WasmInfo {
        imports,
        ei_check,
        memory_grow_flag,
        has_format: true,
        report,
    })
}

fn parse_export_section(
    export_section: ExportSectionReader,
    view_endpoints: &[String],
) -> HashMap<usize, String> {
    let mut views_data: HashMap<usize, String> = HashMap::new();
    for export in export_section {
        let export = export.expect("Failed to read export section");
        if let wasmparser::ExternalKind::Func = export.kind {
            if view_endpoints.contains(&export.name.to_string()) {
                views_data.insert(export.index.try_into().unwrap(), export.name.to_string());
            }
        }
    }
    views_data
}

fn mark_write(
    func: usize,
    call_graph: &HashMap<usize, Vec<usize>>,
    write_functions: &mut Vec<usize>,
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
                write_functions.push(func);
            } else {
                mark_write(callee, call_graph, write_functions, visited);
                if write_functions.contains(&callee) {
                    write_functions.push(func);
                }
            }
        }
    }
}

fn create_call_graph(body: FunctionBody, call_graph: &mut HashMap<usize, Vec<usize>>) {
    let mut instructions_reader = body
        .get_operators_reader()
        .expect("Failed to get operators reader");

    let mut call_functions = Vec::new();
    while let Ok(op) = instructions_reader.read() {
        if let Operator::Call { function_index } = op {
            let function_usize: usize = function_index.try_into().unwrap();
            call_functions.push(function_usize);
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

pub fn extract_imports(
    import_section: ImportSectionReader,
    import_extraction_enabled: bool,
    call_graph: &mut HashMap<usize, Vec<usize>>,
    write_functions: &mut Vec<usize>,
) -> Vec<String> {
    if !import_extraction_enabled {
        return Vec::new();
    }

    let mut import_names = Vec::new();
    for (index, import) in import_section.into_iter().flatten().enumerate() {
        import_names.push(import.name.to_string());
        call_graph.insert(index, vec![]);
        if WRITE_OP.contains(&import.name) {
            write_functions.push(index);
        }
    }

    import_names.sort();

    import_names
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
