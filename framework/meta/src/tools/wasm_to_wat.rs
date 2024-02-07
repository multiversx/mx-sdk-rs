use std::fs;

/// Converts a .wasm file on the disk to .wat.
pub fn wasm_to_wat(output_wasm_path: &str, output_wat_path: &str) {
    let wat_string =
        wasmprinter::print_file(output_wasm_path).expect("could not convert wasm to wat");
    fs::write(output_wat_path, wat_string).expect("could not write wat file");
}
