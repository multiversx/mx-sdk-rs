use colored::Colorize;
use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};
use wasmparser::{
    BinaryReaderError, DataSectionReader, ExportSectionReader, FunctionBody, ImportSectionReader,
    Operator, Parser, Payload,
};

use crate::ei::EIVersion;

use super::report_creator::ReportCreator;

type CallGraph = HashMap<usize, HashSet<usize>>;

const ERROR_FAIL_ALLOCATOR: &[u8; 27] = b"memory allocation forbidden";
const WRITE_OP: &[&str] = &[
    "mBufferStorageStore",
    "storageStore",
    "int64storageStore",
    "bigIntStorageStoreUnsigned",
    "smallIntStorageStoreUnsigned",
    "smallIntStorageStoreSigned",
];

#[derive(Default)]
pub struct WasmInfo {
    pub imports: Vec<String>,
    pub ei_check: bool,
    pub memory_grow_flag: bool,
    pub report: ReportCreator,
    pub call_graph: CallGraph,
    pub write_index_functions: HashSet<usize>,
    pub view_endpoints: HashMap<String, usize>,
}

impl WasmInfo {
    pub fn extract_wasm_info(
        output_wasm_path: &PathBuf,
        extract_imports_enabled: bool,
        check_ei: &Option<EIVersion>,
        view_endpoints: Vec<&str>,
    ) -> WasmInfo {
        let wasm_data = fs::read(output_wasm_path)
            .expect("error occurred while extracting information from .wasm: file not found");

        let wasm_info = populate_wasm_info(
            output_wasm_path,
            wasm_data,
            extract_imports_enabled,
            check_ei,
            view_endpoints,
        );

        wasm_info.expect("error occurred while extracting information from .wasm file")
    }

    fn create_call_graph(&mut self, body: FunctionBody) {
        let mut instructions_reader = body
            .get_operators_reader()
            .expect("Failed to get operators reader");

        let mut call_functions = HashSet::new();
        while let Ok(op) = instructions_reader.read() {
            if !is_whitelisted(&op) {
                panic!(
                    "{}",
                    "Operator not supported for VM execution"
                        .to_string()
                        .red()
                        .bold()
                );
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
                self.imports.push(import.name.to_string());
            }
            self.call_graph.insert(index, HashSet::new());
            if WRITE_OP.contains(&import.name) {
                self.write_index_functions.insert(index);
            }
        }

        self.imports.sort();
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
    wasm_data: Vec<u8>,
    import_extraction_enabled: bool,
    check_ei: &Option<EIVersion>,
    view_endpoints: Vec<&str>,
) -> Result<WasmInfo, BinaryReaderError> {
    let mut wasm_info = WasmInfo::default();

    let parser = Parser::new(0);
    for payload in parser.parse_all(&wasm_data) {
        match payload? {
            Payload::ImportSection(import_section) => {
                wasm_info.process_imports(import_section, import_extraction_enabled);
                wasm_info.ei_check |= is_ei_valid(&wasm_info.imports, check_ei);
            },
            Payload::DataSection(data_section) => {
                wasm_info.report.has_allocator |= is_fail_allocator_triggered(data_section.clone());
                wasm_info.report.has_panic.max_severity(data_section);
            },
            Payload::CodeSectionEntry(code_section) => {
                wasm_info.memory_grow_flag |= is_mem_grow(&code_section);
                wasm_info.create_call_graph(code_section);
            },
            Payload::ExportSection(export_section) => {
                wasm_info.parse_export_section(export_section, &view_endpoints);
            },
            _ => (),
        }
    }

    wasm_info.detect_write_operations_in_views();

    let report = ReportCreator {
        path: path.to_path_buf(),
        has_allocator: wasm_info.report.has_allocator,
        has_panic: wasm_info.report.has_panic,
    };

    Ok(WasmInfo {
        imports: wasm_info.imports,
        ei_check: wasm_info.ei_check,
        memory_grow_flag: wasm_info.memory_grow_flag,
        call_graph: wasm_info.call_graph,
        report,
        write_index_functions: wasm_info.write_index_functions,
        view_endpoints: wasm_info.view_endpoints,
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

fn is_whitelisted(op: &Operator) -> bool {
    match op {
        Operator::Block { .. } => true,
        Operator::Br { .. } => true,
        Operator::BrIf { .. } => true,
        Operator::BrTable { .. } => true,
        Operator::Call { .. } => true,
        Operator::CallIndirect { .. } => true,
        Operator::Catch { .. } => true,
        Operator::CatchAll { .. } => true,
        Operator::Delegate { .. } => true,
        Operator::Drop { .. } => true,
        Operator::Else { .. } => true,
        Operator::End { .. } => true,
        Operator::GlobalGet { .. } => true,
        Operator::GlobalSet { .. } => true,
        Operator::I32Add { .. } => true,
        Operator::I32And { .. } => true,
        Operator::I32Clz { .. } => true,
        Operator::I32Const { .. } => true,
        Operator::I32Ctz { .. } => true,
        Operator::I32DivS { .. } => true,
        Operator::I32DivU { .. } => true,
        Operator::I32Eq { .. } => true,
        Operator::I32Eqz { .. } => true,
        Operator::I32Extend16S { .. } => true,
        Operator::I32Extend8S { .. } => true,
        Operator::I32GeS { .. } => true,
        Operator::I32GeU { .. } => true,
        Operator::I32GtS { .. } => true,
        Operator::I32GtU { .. } => true,
        Operator::I32LeS { .. } => true,
        Operator::I32LeU { .. } => true,
        Operator::I32Load { .. } => true,
        Operator::I32Load16S { .. } => true,
        Operator::I32Load16U { .. } => true,
        Operator::I32Load8S { .. } => true,
        Operator::I32Load8U { .. } => true,
        Operator::I32LtS { .. } => true,
        Operator::I32LtU { .. } => true,
        Operator::I32Mul { .. } => true,
        Operator::I32Ne { .. } => true,
        Operator::I32Or { .. } => true,
        Operator::I32Popcnt { .. } => true,
        Operator::I32RemS { .. } => true,
        Operator::I32RemU { .. } => true,
        Operator::I32Rotl { .. } => true,
        Operator::I32Rotr { .. } => true,
        Operator::I32Shl { .. } => true,
        Operator::I32ShrS { .. } => true,
        Operator::I32ShrU { .. } => true,
        Operator::I32Store { .. } => true,
        Operator::I32Store16 { .. } => true,
        Operator::I32Store8 { .. } => true,
        Operator::I32Sub { .. } => true,
        Operator::I32WrapI64 { .. } => true,
        Operator::I32Xor { .. } => true,
        Operator::I64Add { .. } => true,
        Operator::I64And { .. } => true,
        Operator::I64Clz { .. } => true,
        Operator::I64Const { .. } => true,
        Operator::I64Ctz { .. } => true,
        Operator::I64DivS { .. } => true,
        Operator::I64DivU { .. } => true,
        Operator::I64Eq { .. } => true,
        Operator::I64Eqz { .. } => true,
        Operator::I64Extend16S { .. } => true,
        Operator::I64Extend32S { .. } => true,
        Operator::I64Extend8S { .. } => true,
        Operator::I64ExtendI32S { .. } => true,
        Operator::I64ExtendI32U { .. } => true,
        Operator::I64GeS { .. } => true,
        Operator::I64GeU { .. } => true,
        Operator::I64GtS { .. } => true,
        Operator::I64GtU { .. } => true,
        Operator::I64LeS { .. } => true,
        Operator::I64LeU { .. } => true,
        Operator::I64Load { .. } => true,
        Operator::I64Load16S { .. } => true,
        Operator::I64Load16U { .. } => true,
        Operator::I64Load32S { .. } => true,
        Operator::I64Load32U { .. } => true,
        Operator::I64Load8S { .. } => true,
        Operator::I64Load8U { .. } => true,
        Operator::I64LtS { .. } => true,
        Operator::I64LtU { .. } => true,
        Operator::I64Mul { .. } => true,
        Operator::I64Ne { .. } => true,
        Operator::I64Or { .. } => true,
        Operator::I64Popcnt { .. } => true,
        Operator::I64RemS { .. } => true,
        Operator::I64RemU { .. } => true,
        Operator::I64Rotl { .. } => true,
        Operator::I64Rotr { .. } => true,
        Operator::I64Shl { .. } => true,
        Operator::I64ShrS { .. } => true,
        Operator::I64ShrU { .. } => true,
        Operator::I64Store { .. } => true,
        Operator::I64Store16 { .. } => true,
        Operator::I64Store32 { .. } => true,
        Operator::I64Store8 { .. } => true,
        Operator::I64Sub { .. } => true,
        Operator::I64Xor { .. } => true,
        Operator::If { .. } => true,
        Operator::LocalGet { .. } => true,
        Operator::LocalSet { .. } => true,
        Operator::LocalTee { .. } => true,
        // Operator::LocalAllocate { .. } => true,
        Operator::Loop { .. } => true,
        Operator::MemoryGrow { .. } => true,
        Operator::MemorySize { .. } => true,
        Operator::Nop { .. } => true,
        Operator::RefFunc { .. } => true,
        Operator::RefIsNull { .. } => true,
        Operator::RefNull { .. } => true,
        Operator::Rethrow { .. } => true,
        Operator::Return { .. } => true,
        Operator::ReturnCall { .. } => true,
        Operator::ReturnCallIndirect { .. } => true,
        Operator::Select { .. } => true,
        Operator::TableGet { .. } => true,
        Operator::TableGrow { .. } => true,
        Operator::TableInit { .. } => true,
        Operator::TableSet { .. } => true,
        Operator::TableSize { .. } => true,
        Operator::Throw { .. } => true,
        Operator::Try { .. } => true,
        Operator::TypedSelect { .. } => true,
        Operator::Unreachable { .. } => true,
        // Operator::Unwind { .. } => true,
        _ => false,
    }
}
