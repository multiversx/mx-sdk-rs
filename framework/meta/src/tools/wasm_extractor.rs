use colored::Colorize;
use std::fs;
use wasmparser::{BinaryReaderError, Parser, Payload};

const ERROR_FAIL_ALLOCATOR: &[u8; 27] = b"memory allocation forbidden";

pub struct WasmInfo {
    pub imports: Vec<String>,
    pub allocator_trigger: bool,
}

impl WasmInfo {
    const fn new() -> Self {
        WasmInfo {
            imports: Vec::new(),
            allocator_trigger: false,
        }
    }

    pub fn set_imports(&mut self, output_wasm_path: &str) {
        let wasm_data = fs::read(output_wasm_path)
            .expect("error occured while extracting imports from .wasm: file not found");

        let imports = parse_wasm_imports(wasm_data)
            .expect("error occured while extracting imports from .wasm ");

        self.imports = imports;
    }

    pub fn build_wasm_info(output_wasm_path: &str) -> Self {
        let wasm_data = fs::read(output_wasm_path).expect(
            "error occured while extracting memory allocation information from .wasm: file not found",
        );

        let allocator_trigger = is_memory_allocation(wasm_data)
            .expect("error occured while extracting memory allocation information from .wasm ");

        if allocator_trigger {
            println!(
                "{}",
                "FailAllocator triggered: memory allocation forbidden"
                    .to_string()
                    .red()
                    .bold()
            );
        }

        Self::new()
    }
}

fn parse_wasm_imports(wasm_data: Vec<u8>) -> Result<Vec<String>, BinaryReaderError> {
    let mut import_names = Vec::new();

    let parser = Parser::new(0);
    for payload in parser.parse_all(&wasm_data) {
        if let Payload::ImportSection(import_section) = payload? {
            for import in import_section {
                import_names.push(import?.name.to_string());
            }
        }
    }

    import_names.sort();

    Ok(import_names)
}

pub fn is_memory_allocation(wasm_data: Vec<u8>) -> Result<bool, BinaryReaderError> {
    let parser = Parser::new(0);

    for payload in parser.parse_all(&wasm_data).flatten() {
        if let Payload::DataSection(data_section) = payload {
            for data_fragment in data_section {
                if data_fragment?
                    .data
                    .windows(ERROR_FAIL_ALLOCATOR.len())
                    .any(|data| data == ERROR_FAIL_ALLOCATOR)
                {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}
