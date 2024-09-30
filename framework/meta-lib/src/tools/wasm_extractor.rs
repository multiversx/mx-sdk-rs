use colored::Colorize;
use std::fs;
use wasmparser::{
    BinaryReaderError, DataSectionReader, FunctionBody, ImportSectionReader, Operator, Parser,
    Payload,
};

use crate::ei::EIVersion;

use super::{panic_report::PanicReport, report_creator::ReportCreator};

const ERROR_FAIL_ALLOCATOR: &[u8; 27] = b"memory allocation forbidden";

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
    ) -> Result<WasmInfo, BinaryReaderError> {
        let wasm_data = fs::read(output_wasm_path)
            .expect("error occured while extracting information from .wasm: file not found");

        populate_wasm_info(
            output_wasm_path.to_string(),
            wasm_data,
            extract_imports_enabled,
            check_ei,
        )
    }
}

pub(crate) fn populate_wasm_info(
    path: String,
    wasm_data: Vec<u8>,
    extract_imports_enabled: bool,
    check_ei: &Option<EIVersion>,
) -> Result<WasmInfo, BinaryReaderError> {
    let mut imports = Vec::new();
    let mut allocator_trigger = false;
    let mut ei_check = false;
    let mut memory_grow_flag = false;
    let mut has_panic: PanicReport = PanicReport::default();

    let parser = Parser::new(0);
    for payload in parser.parse_all(&wasm_data) {
        match payload? {
            Payload::ImportSection(import_section) => {
                imports = extract_imports(import_section, extract_imports_enabled);
                ei_check |= is_ei_valid(imports.clone(), check_ei);
            },
            Payload::DataSection(data_section) => {
                allocator_trigger |= is_fail_allocator_triggered(data_section.clone());
                has_panic.max_severity(data_section);
            },
            Payload::CodeSectionEntry(code_section) => {
                memory_grow_flag |= is_mem_grow(code_section);
            },
            _ => (),
        }
    }

    let report = ReportCreator {
        path,
        has_allocator: allocator_trigger,
        has_panic,
    };

    Ok(WasmInfo {
        imports,
        ei_check,
        memory_grow_flag,
        has_format: true,
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

pub fn extract_imports(
    import_section: ImportSectionReader,
    import_extraction_enabled: bool,
) -> Vec<String> {
    if !import_extraction_enabled {
        return Vec::new();
    }

    let mut import_names = Vec::new();
    for import in import_section.into_iter().flatten() {
        import_names.push(import.name.to_string());
    }

    import_names.sort();

    import_names
}

fn is_ei_valid(imports: Vec<String>, check_ei: &Option<EIVersion>) -> bool {
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

fn is_mem_grow(code_section: FunctionBody) -> bool {
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
