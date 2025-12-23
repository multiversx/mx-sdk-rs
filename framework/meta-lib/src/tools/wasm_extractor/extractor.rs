use colored::Colorize;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};
use wasmparser::{
    BinaryReaderError, CompositeInnerType, DataSectionReader, ElementItems, ElementSectionReader,
    ExportSectionReader, FunctionBody, ImportSectionReader, Operator, Parser, Payload, TypeRef,
    TypeSectionReader, ValType,
};

use crate::{ei::EIVersion, tools::CodeReport};

use super::{
    CallGraph, EndpointInfo, FunctionInfo, OpcodeVersion, opcode_whitelist::is_opcode_whitelisted,
    report::WasmReport,
};

const ERROR_FAIL_ALLOCATOR: &[u8; 27] = b"memory allocation forbidden";
const WRITE_OP: &[&str] = &[
    "mBufferStorageStore",
    "storageStore",
    "int64storageStore",
    "bigIntStorageStoreUnsigned",
    "smallIntStorageStoreUnsigned",
    "smallIntStorageStoreSigned",
];

#[derive(Default, Debug, Clone)]
pub struct WasmInfo {
    pub call_graph: CallGraph,
    pub write_index_functions: HashSet<usize>,
    pub report: WasmReport,
    pub data: Vec<u8>,
    pub func_types: HashMap<u32, FunctionType>,
}

#[derive(Debug, Clone)]
pub struct FunctionType {
    pub params: Vec<ValType>,
    pub returns: Vec<ValType>,
}

impl WasmInfo {
    pub fn extract_wasm_report(
        output_wasm_path: &PathBuf,
        extract_imports_enabled: bool,
        check_ei: Option<&EIVersion>,
        endpoints: &HashMap<&str, bool>,
        opcode_version: OpcodeVersion,
    ) -> WasmReport {
        let wasm_data = fs::read(output_wasm_path)
            .expect("error occurred while extracting information from .wasm: file not found");

        let wasm_info = WasmInfo::default()
            .add_endpoints(endpoints)
            .add_path(output_wasm_path)
            .add_wasm_data(&wasm_data)
            .populate_wasm_info(extract_imports_enabled, check_ei, opcode_version)
            .expect("error occurred while extracting information from .wasm file");

        wasm_info.report
    }

    pub(crate) fn populate_wasm_info(
        self,
        import_extraction_enabled: bool,
        check_ei: Option<&EIVersion>,
        opcode_version: OpcodeVersion,
    ) -> Result<WasmInfo, BinaryReaderError> {
        let parser = Parser::new(0);
        let mut wasm_info = self.clone();

        for payload in parser.parse_all(&self.data) {
            match payload? {
                Payload::TypeSection(type_section) => {
                    wasm_info.parse_type_section(type_section);
                }
                Payload::ImportSection(import_section) => {
                    wasm_info.process_imports(import_section, import_extraction_enabled);
                    wasm_info.report.ei_check |= is_ei_valid(&wasm_info.report.imports, check_ei);
                }
                Payload::DataSection(data_section) => {
                    wasm_info.report.code.has_allocator |=
                        is_fail_allocator_triggered(data_section.clone());
                    wasm_info.report.code.has_panic.max_severity(data_section);
                }
                Payload::CodeSectionEntry(code_section) => {
                    wasm_info.report.memory_grow_flag |= is_mem_grow(&code_section);
                    wasm_info.create_call_graph(code_section, opcode_version);
                }
                Payload::ExportSection(export_section) => {
                    wasm_info.parse_export_section(export_section);
                }
                Payload::ElementSection(elem_section) => {
                    wasm_info.parse_element_section(elem_section);
                }
                _ => {}
            }
        }

        wasm_info
            .call_graph
            .populate_accessible_from_function_indexes();
        wasm_info
            .call_graph
            .populate_accessible_from_call_indirect();
        wasm_info.call_graph.populate_function_endpoints();
        wasm_info
            .call_graph
            .populate_call_indirect_accessible_from_endpoints();

        wasm_info.detect_write_operations_in_views();
        wasm_info.detect_forbidden_opcodes();

        Ok(wasm_info)
    }

    fn parse_type_section(&mut self, type_section: TypeSectionReader) {
        for (ty_index, ty_result) in type_section.into_iter().enumerate() {
            let rec_group = ty_result.expect("Failed to read type section");
            for sub_type in rec_group.into_types() {
                if let CompositeInnerType::Func(func_ty) = sub_type.composite_type.inner {
                    let ft = FunctionType {
                        params: func_ty.params().to_vec(),
                        returns: func_ty.results().to_vec(),
                    };
                    self.func_types.insert(ty_index as u32, ft);
                }
            }
        }
    }

    pub(crate) fn add_endpoints(mut self, endpoints: &HashMap<&str, bool>) -> Self {
        for (name, readonly) in endpoints {
            self.call_graph
                .endpoints
                .insert(name.to_string(), EndpointInfo::default(*readonly));
        }

        self
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

    fn create_call_graph(&mut self, body: FunctionBody, opcode_version: OpcodeVersion) {
        let mut instructions_reader = body
            .get_operators_reader()
            .expect("Failed to get operators reader");

        let mut function_info = FunctionInfo::new();
        let function_index = self.call_graph.next_function_index();
        while let Ok(op) = instructions_reader.read() {
            match op {
                Operator::Call { function_index } => {
                    function_info.add_called_function(function_index as usize);
                }
                Operator::CallIndirect { .. } => {
                    function_info.contains_call_indirect = true;
                }
                _ => {}
            }

            if !is_opcode_whitelisted(&op, opcode_version) {
                let opcode = extract_opcode(op);
                function_info.add_forbidden_opcode(opcode);
            }
        }

        self.call_graph
            .insert_function(function_index, function_info);
    }

    fn process_imports(
        &mut self,
        import_section: ImportSectionReader,
        import_extraction_enabled: bool,
    ) {
        let signature_map = crate::ei::vm_hook_signature_map();

        for (index, import) in import_section.into_iter().flatten().enumerate() {
            if let TypeRef::Func(type_index) = &import.ty {
                let func_type = self
                    .func_types
                    .get(type_index)
                    .expect("invalid wasm function type index");

                crate::ei::check_vm_hook_signatures(
                    import.name,
                    &func_type.params,
                    &func_type.returns,
                    &signature_map,
                );
                if import_extraction_enabled {
                    self.report.imports.push(import.name.to_string());
                }
                self.call_graph.insert_function(index, FunctionInfo::new());
                if WRITE_OP.contains(&import.name) {
                    self.write_index_functions.insert(index);
                }
            }
        }

        self.report.imports.sort();
    }

    fn detect_write_operations_in_views(&mut self) {
        let mut visited: HashSet<usize> = HashSet::new();

        for index in get_view_endpoints_indexes(&self.call_graph.endpoints) {
            mark_write(self, index, &mut visited);
        }

        for (name, index) in get_view_endpoints(&self.call_graph.endpoints) {
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
        for (&func_index, func_info) in &self.call_graph.function_map {
            if func_info.forbidden_opcodes.is_empty() {
                continue;
            }

            let opcodes = func_info
                .forbidden_opcodes
                .iter()
                .cloned()
                .collect::<Vec<String>>()
                .join(", ");
            let mut message =
                format!("Forbidden opcodes detected in function {func_index}: {opcodes}.");

            let endpoints = self
                .call_graph
                .function_accessible_from_endpoints(func_index);
            if !endpoints.is_empty() {
                message.push_str(&format!(
                    " This function is accessible endpoints: {}.",
                    endpoints
                        .iter()
                        .cloned()
                        .collect::<Vec<String>>()
                        .join(", ")
                ));
            }
            for endpoint in endpoints {
                for forbidden_opcode in &func_info.forbidden_opcodes {
                    self.report.add_forbidden_opcode_accessible_from_endpoint(
                        endpoint.clone(),
                        forbidden_opcode.clone(),
                    );
                }
            }

            if func_info.accessible_from_call_indirect {
                for endpoint in &self.call_graph.call_indirect_accessible_from_endpoints {
                    for forbidden_opcode in &func_info.forbidden_opcodes {
                        self.report.add_forbidden_opcode_accessible_from_endpoint(
                            endpoint.clone(),
                            forbidden_opcode.clone(),
                        );
                    }
                }
                message.push_str(&format!(
                    " This function is accessible via call_indirect, from endpoints: {}.",
                    self.call_graph
                        .call_indirect_accessible_from_endpoints
                        .iter()
                        .cloned()
                        .collect::<Vec<String>>()
                        .join(", ")
                ));
            }

            println!("{}", message.red().bold());
        }
    }

    fn parse_export_section(&mut self, export_section: ExportSectionReader) {
        if self.call_graph.endpoints.is_empty() {
            return;
        }

        for export in export_section {
            let export = export.expect("Failed to read export section");
            if wasmparser::ExternalKind::Func == export.kind {
                if let Some(endpoint) = self.call_graph.endpoints.get_mut(export.name) {
                    endpoint.set_index(export.index.try_into().unwrap());
                }
            }
        }
    }

    fn parse_element_section(&mut self, element_section: ElementSectionReader) {
        for t in element_section.into_iter() {
            let element = t.expect("Failed to read table section");

            if let ElementItems::Functions(functions) = element.items {
                for func_result in functions {
                    let function_index =
                        func_result.expect("Failed to read function index in element section");
                    self.call_graph
                        .table_functions
                        .push(function_index as usize);
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

    let callees: Vec<usize> = if let Some(callees) = wasm_info.call_graph.function_map.get(&func) {
        callees.called_function_indexes.iter().cloned().collect()
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

fn is_ei_valid(imports: &[String], check_ei: Option<&EIVersion>) -> bool {
    if let Some(ei) = check_ei {
        let mut num_errors = 0;
        for import in imports {
            if !ei.contains_vm_hook(import.as_str()) {
                num_errors += 1;
            }

            if let Some(deprecated) = ei.deprecated_vm_hook(import) {
                panic!(
                    "{} {} - {}",
                    "Deprecated VM hook used:".to_string().yellow().bold(),
                    deprecated.name.yellow().bold(),
                    deprecated.note.yellow()
                );
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
