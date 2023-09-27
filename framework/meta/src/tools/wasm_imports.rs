use anyhow::Result;
use std::fs;
use wasmparser::{Parser, Payload};

/// Parses the WebAssembly code and extracts all the import names.
pub fn extract_wasm_imports(output_wasm_path: &str) -> Result<Vec<String>> {
    let wasm_data = fs::read(output_wasm_path)?;
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
