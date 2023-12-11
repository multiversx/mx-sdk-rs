use std::fs;
use wasmparser::{BinaryReaderError, Parser, Payload};

/// Parses the WebAssembly code and extracts all the import names.
pub fn extract_wasm_imports(output_wasm_path: &str) -> Vec<String> {
    let wasm_data = fs::read(output_wasm_path)
        .expect("error occured while extracting imports from .wasm: file not found");

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

    Ok(import_names)
}
