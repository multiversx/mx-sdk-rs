use colored::Colorize;
use std::fs;
use wasmparser::{BinaryReaderError, DataSectionReader, ImportSectionReader, Parser, Payload};

const ERROR_FAIL_ALLOCATOR: &[u8; 27] = b"memory allocation forbidden";

pub struct WasmInfo {
    pub imports: Vec<String>,
    pub allocator_trigger: bool,
}

impl WasmInfo {
    const fn new(imports: Vec<String>, allocator_trigger: bool) -> Self {
        WasmInfo {
            imports,
            allocator_trigger,
        }
    }

    pub fn extract_wasm_info(
        output_wasm_path: &str,
        extract_imports_enabled: bool,
    ) -> Result<WasmInfo, BinaryReaderError> {
        let wasm_data = fs::read(output_wasm_path)
            .expect("error occured while extracting information from .wasm: file not found");

        populate_wasm_info(wasm_data, extract_imports_enabled)
    }
}

fn populate_wasm_info(
    wasm_data: Vec<u8>,
    extract_imports_enabled: bool,
) -> Result<WasmInfo, BinaryReaderError> {
    let mut imports = Vec::new();
    let mut allocator_triggered = false;

    let parser = Parser::new(0);
    for payload in parser.parse_all(&wasm_data) {
        match payload? {
            Payload::ImportSection(import_section) => {
                imports = extract_imports(import_section, extract_imports_enabled);
            },
            Payload::DataSection(data_section) => {
                allocator_triggered = is_fail_allocator_triggered(data_section);
            },
            _ => (),
        }
    }

    Ok(WasmInfo::new(imports, allocator_triggered))
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
