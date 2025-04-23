use colored::Colorize;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};
use wasmparser::{
    BinaryReaderError, DataSectionReader, ExportSectionReader, FunctionBody, ImportSectionReader,
    Operator, Parser, Payload,
};

use crate::{ei::EIVersion, tools::CodeReport};

use super::{
    report::WasmReport,
    whitelisted_opcodes::{is_whitelisted, ERROR_FAIL_ALLOCATOR, WRITE_OP},
};

type CallGraph = HashMap<usize, HashSet<usize>>;

#[derive(Default)]
pub struct WasmInfo {
    pub call_graph: CallGraph,
    pub write_index_functions: HashSet<usize>,
    pub view_endpoints: HashMap<String, usize>,
    pub report: WasmReport,
}

impl WasmInfo {
    pub fn extract_wasm_report(
        output_wasm_path: &PathBuf,
        extract_imports_enabled: bool,
        check_ei: Option<&EIVersion>,
        view_endpoints: &[&str],
    ) -> WasmReport {
        let wasm_data = fs::read(output_wasm_path)
            .expect("error occurred while extracting information from .wasm: file not found");

        let wasm_info = populate_wasm_info(
            output_wasm_path,
            &wasm_data,
            extract_imports_enabled,
            check_ei,
            view_endpoints,
        )
        .expect("error occurred while extracting information from .wasm file");

        wasm_info.report
    }

    fn create_call_graph(&mut self, body: FunctionBody) {
        let mut instructions_reader = body
            .get_operators_reader()
            .expect("Failed to get operators reader");

        let mut call_functions = HashSet::new();
        while let Ok(op) = instructions_reader.read() {
            if !is_whitelisted(&op) {
                let op_str = format!("{:?}", op);
                let op_vec: Vec<&str> = op_str.split_whitespace().collect();
                self.report.forbidden_opcodes.push(op_vec[0].to_owned());
            }

            if let Operator::Call { function_index } = op {
                let function_usize: usize = function_index.try_into().unwrap();
                call_functions.insert(function_usize);
            }
        }

        self.call_graph
            .insert(self.call_graph.len(), call_functions);
    }

    pub fn process_imports(
        &mut self,
        import_section: ImportSectionReader,
        import_extraction_enabled: bool,
    ) {
        for (index, import) in import_section.into_iter().flatten().enumerate() {
            if import_extraction_enabled {
                self.report.imports.push(import.name.to_string());
            }
            self.call_graph.insert(index, HashSet::new());
            if WRITE_OP.contains(&import.name) {
                self.write_index_functions.insert(index);
            }
        }

        self.report.imports.sort();
    }

    pub fn detect_write_operations_in_views(&mut self) {
        let mut visited: HashSet<usize> = HashSet::new();
        for index in self.view_endpoints.values() {
            mark_write(
                *index,
                &self.call_graph,
                &mut self.write_index_functions,
                &mut visited,
            );
        }

        for (name, index) in &self.view_endpoints {
            if self.write_index_functions.contains(index) {
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

    pub fn detect_forbidden_ops(&mut self) {}

    fn parse_export_section(
        &mut self,
        export_section: ExportSectionReader,
        view_endpoints: &[&str],
    ) {
        for export in export_section {
            let export = export.expect("Failed to read export section");
            if let wasmparser::ExternalKind::Func = export.kind {
                if view_endpoints.contains(&export.name) {
                    self.view_endpoints
                        .insert(export.name.to_owned(), export.index.try_into().unwrap());
                }
            }
        }
    }
}

pub(crate) fn populate_wasm_info(
    path: &Path,
    wasm_data: &[u8],
    import_extraction_enabled: bool,
    check_ei: Option<&EIVersion>,
    view_endpoints: &[&str],
) -> Result<WasmInfo, BinaryReaderError> {
    let mut wasm_info = WasmInfo::default();

    let parser = Parser::new(0);
    for payload in parser.parse_all(wasm_data) {
        match payload? {
            Payload::ImportSection(import_section) => {
                wasm_info.process_imports(import_section, import_extraction_enabled);
                wasm_info.report.ei_check |= is_ei_valid(&wasm_info.report.imports, check_ei);
            },
            Payload::DataSection(data_section) => {
                wasm_info.report.code.has_allocator |=
                    is_fail_allocator_triggered(data_section.clone());
                wasm_info.report.code.has_panic.max_severity(data_section);
            },
            Payload::CodeSectionEntry(code_section) => {
                wasm_info.report.memory_grow_flag |= is_mem_grow(&code_section);
                wasm_info.create_call_graph(code_section);
            },
            Payload::ExportSection(export_section) => {
                wasm_info.parse_export_section(export_section, view_endpoints);
            },
            _ => (),
        }
    }

    wasm_info.detect_write_operations_in_views();

    let report = WasmReport {
        imports: wasm_info.report.imports,
        memory_grow_flag: wasm_info.report.memory_grow_flag,
        ei_check: wasm_info.report.ei_check,
        code: CodeReport {
            path: path.to_path_buf(),
            has_allocator: wasm_info.report.code.has_allocator,
            has_panic: wasm_info.report.code.has_panic,
        },
        forbidden_opcodes: wasm_info.report.forbidden_opcodes,
    };

    Ok(WasmInfo {
        call_graph: wasm_info.call_graph,
        write_index_functions: wasm_info.write_index_functions,
        view_endpoints: wasm_info.view_endpoints,
        report,
    })
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

fn is_ei_valid(imports: &[String], check_ei: Option<&EIVersion>) -> bool {
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
    let mut instructions_reader = code_section
        .get_operators_reader()
        .expect("Failed to get operators reader");

    while let Ok(op) = instructions_reader.read() {
        if let Operator::MemoryGrow { mem: _ } = op {
            return true;
        }
    }

    false
}
