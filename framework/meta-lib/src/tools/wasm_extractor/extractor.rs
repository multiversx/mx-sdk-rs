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
    endpoint_info::{EndpointInfo, FunctionInfo},
    report::WasmReport,
    whitelisted_opcodes::{is_whitelisted, ERROR_FAIL_ALLOCATOR, WRITE_OP},
};

type CallGraph = HashMap<usize, FunctionInfo>;

#[derive(Default, Debug, Clone)]
pub struct WasmInfo {
    pub call_graph: CallGraph,
    pub write_index_functions: HashSet<usize>,
    pub endpoints: HashMap<String, EndpointInfo>,
    pub report: WasmReport,
    pub data: Vec<u8>,
}

impl WasmInfo {
    pub fn extract_wasm_report(
        output_wasm_path: &PathBuf,
        extract_imports_enabled: bool,
        check_ei: Option<&EIVersion>,
        endpoints: &HashMap<&str, bool>,
    ) -> WasmReport {
        let wasm_data = fs::read(output_wasm_path)
            .expect("error occurred while extracting information from .wasm: file not found");

        let wasm_info = WasmInfo::default()
            .add_endpoints(endpoints)
            .add_path(output_wasm_path)
            .add_wasm_data(&wasm_data)
            .populate_wasm_info(extract_imports_enabled, check_ei)
            .expect("error occurred while extracting information from .wasm file");

        wasm_info.report
    }

    pub(crate) fn populate_wasm_info(
        self,
        import_extraction_enabled: bool,
        check_ei: Option<&EIVersion>,
    ) -> Result<WasmInfo, BinaryReaderError> {
        let parser = Parser::new(0);
        let mut wasm_info = self.clone();

        for payload in parser.parse_all(&self.data) {
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
                    wasm_info.parse_export_section(export_section);
                },
                _ => (),
            }
        }

        wasm_info.detect_write_operations_in_views();
        wasm_info.detect_forbidden_opcodes();

        Ok(wasm_info)
    }

    pub(crate) fn add_endpoints(self, endpoints: &HashMap<&str, bool>) -> Self {
        let mut endpoints_map = HashMap::new();

        for (name, readonly) in endpoints {
            endpoints_map.insert(name.to_string(), EndpointInfo::default(*readonly));
        }

        WasmInfo {
            endpoints: endpoints_map,
            ..self
        }
    }

    pub(crate) fn add_wasm_data(self, data: &[u8]) -> Self {
        WasmInfo {
            data: data.to_vec(),
            ..self
        }
    }

    fn add_path(self, path: &Path) -> Self {
        WasmInfo {
            report: WasmReport {
                code: CodeReport {
                    path: path.to_path_buf(),
                    ..self.report.code
                },
                ..self.report
            },
            ..self
        }
    }

    fn create_call_graph(&mut self, body: FunctionBody) {
        let mut instructions_reader = body
            .get_operators_reader()
            .expect("Failed to get operators reader");

        let mut function_info = FunctionInfo::new();
        while let Ok(op) = instructions_reader.read() {
            if let Operator::Call { function_index } = op {
                let function_usize: usize = function_index.try_into().unwrap();
                function_info.add_function_index(function_usize);
            }

            if !is_whitelisted(&op) {
                let opcode = extract_opcode(op);
                function_info.add_forbidden_opcode(opcode);
            }
        }

        self.call_graph.insert(self.call_graph.len(), function_info);
    }

    fn process_imports(
        &mut self,
        import_section: ImportSectionReader,
        import_extraction_enabled: bool,
    ) {
        for (index, import) in import_section.into_iter().flatten().enumerate() {
            if import_extraction_enabled {
                self.report.imports.push(import.name.to_string());
            }
            self.call_graph.insert(index, FunctionInfo::new());
            if WRITE_OP.contains(&import.name) {
                self.write_index_functions.insert(index);
            }
        }

        self.report.imports.sort();
    }

    fn detect_write_operations_in_views(&mut self) {
        let mut visited: HashSet<usize> = HashSet::new();

        for index in get_view_endpoints_indexes(&self.endpoints) {
            mark_write(self, index, &mut visited);
        }

        for (name, index) in get_view_endpoints(&self.endpoints) {
            if self.write_index_functions.contains(&index) {
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

    fn detect_forbidden_opcodes(&mut self) {
        let mut visited: HashSet<usize> = HashSet::new();
        for endpoint_info in self.endpoints.values_mut() {
            mark_forbidden_functions(endpoint_info.index, &mut self.call_graph, &mut visited);
            endpoint_info.forbidden_opcodes = self
                .call_graph
                .get(&endpoint_info.index)
                .unwrap()
                .forbidden_opcodes
                .clone();
        }

        for (name, endpoint_info) in &self.endpoints {
            if !endpoint_info.forbidden_opcodes.is_empty() {
                self.report.forbidden_opcodes.insert(
                    name.to_string(),
                    endpoint_info.forbidden_opcodes.iter().cloned().collect(),
                );

                println!(
                    "{}{}{} {}",
                    "Forbidden opcodes detected in endpoint \""
                        .to_string()
                        .red()
                        .bold(),
                    name.red().bold(),
                    "\". This are the opcodes:".to_string().red().bold(),
                    self.report
                        .forbidden_opcodes
                        .get(name)
                        .unwrap()
                        .join(", ")
                        .red()
                        .bold()
                );
            }
        }
    }

    fn parse_export_section(&mut self, export_section: ExportSectionReader) {
        if self.endpoints.is_empty() {
            return;
        }

        for export in export_section {
            let export = export.expect("Failed to read export section");
            if wasmparser::ExternalKind::Func == export.kind {
                if let Some(endpoint) = self.endpoints.get_mut(export.name) {
                    endpoint.set_index(export.index.try_into().unwrap());
                }
            }
        }
    }
}

pub(crate) fn get_view_endpoints_indexes(endpoints: &HashMap<String, EndpointInfo>) -> Vec<usize> {
    endpoints
        .values()
        .filter(|endpoint_info| endpoint_info.readonly)
        .map(|endpoint_info| endpoint_info.index)
        .collect()
}

pub(crate) fn get_view_endpoints(
    endpoints: &HashMap<String, EndpointInfo>,
) -> HashMap<&str, usize> {
    let mut view_endpoints = HashMap::new();

    for (name, endpoint_info) in endpoints {
        if endpoint_info.readonly {
            view_endpoints.insert(name.as_str(), endpoint_info.index);
        }
    }

    view_endpoints
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

fn mark_write(wasm_info: &mut WasmInfo, func: usize, visited: &mut HashSet<usize>) {
    // Return early to prevent cycles.
    if visited.contains(&func) {
        return;
    }

    visited.insert(func);

    let callees: Vec<usize> = if let Some(callees) = wasm_info.call_graph.get(&func) {
        callees.indexes.iter().cloned().collect()
    } else {
        return;
    };

    for callee in callees {
        if wasm_info.write_index_functions.contains(&callee) {
            wasm_info.write_index_functions.insert(func);
        } else {
            mark_write(wasm_info, callee, visited);
            if wasm_info.write_index_functions.contains(&callee) {
                wasm_info.write_index_functions.insert(func);
            }
        }
    }
}

fn mark_forbidden_functions(func: usize, call_graph: &mut CallGraph, visited: &mut HashSet<usize>) {
    // Return early to prevent cycles.
    if visited.contains(&func) {
        return;
    }

    visited.insert(func);

    if let Some(function_info) = call_graph.get(&func) {
        for index in function_info.indexes.clone() {
            if !call_graph.get(&index).unwrap().forbidden_opcodes.is_empty() {
                let index_forbidden_opcodes =
                    call_graph.get(&index).unwrap().forbidden_opcodes.clone();

                call_graph
                    .get_mut(&func)
                    .unwrap()
                    .add_forbidden_opcodes(index_forbidden_opcodes);
            } else {
                mark_forbidden_functions(index, call_graph, visited);
                if !call_graph.get(&index).unwrap().forbidden_opcodes.is_empty() {
                    let index_forbidden_opcodes =
                        call_graph.get(&index).unwrap().forbidden_opcodes.clone();

                    call_graph
                        .get_mut(&func)
                        .unwrap()
                        .add_forbidden_opcodes(index_forbidden_opcodes);
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

fn extract_opcode(op: Operator) -> String {
    let op_str = format!("{:?}", op);
    let op_vec: Vec<&str> = op_str.split_whitespace().collect();

    op_vec[0].to_owned()
}
