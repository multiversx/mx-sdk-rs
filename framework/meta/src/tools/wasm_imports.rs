use std::fs;
use wasmparser::{BinaryReaderError, Parser, Payload};

/// Parses the WebAssembly code and extracts all the import names.
pub fn extract_wasm_imports(output_wasm_path: &str) -> Vec<String> {
    let wasm_data = fs::read(output_wasm_path)
        .expect("error occured while extracting imports from .wasm: file not found");

    parse_wasm_extract_fail_allocator_error(wasm_data.clone());
    parse_wasm_imports(wasm_data).expect("error occured while extracting imports from .wasm ")
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

fn parse_wasm_extract_fail_allocator_error(wasm_data: Vec<u8>) {
    let parser = Parser::new(0);

    for payload in parser.parse_all(&wasm_data).flatten() {
        if let Payload::DataSection(data_section) = payload {
            for data in data_section.into_iter() {
                match data {
                    Ok(data) => {
                        if let Ok(utf8_str) = std::str::from_utf8(data.data) {
                            if utf8_str.contains("memory allocation forbidden") {
                                panic!("FailAllocator")
                            }
                        }
                    },
                    Err(_err) => {},
                }
            }
        }
    }
}
