use std::{fs, path::Path};

pub const CODEHASH_FILE_SUFFIX: &str = ".codehash.txt";

/// Computes the Blake2b-256 code hash of the `.wasm` file at `wasm_path`,
/// and writes the lowercase hex string to `codehash_path`.
pub fn generate_codehash(wasm_path: &Path, codehash_path: &Path) {
    let wasm_bytes = fs::read(wasm_path)
        .unwrap_or_else(|err| panic!("failed to read wasm file {}: {err}", wasm_path.display()));
    let hash = multiversx_sc::chain_core::std::code_hash(&wasm_bytes);
    let hex_hash = hex::encode(hash);
    fs::write(codehash_path, hex_hash).unwrap_or_else(|err| {
        panic!(
            "failed to write codehash file {}: {err}",
            codehash_path.display()
        )
    });
}

/// Scans `output_dir` for `.wasm` files and generates a codehash file for each one.
///
/// For every `<name>.wasm` found, writes the Blake2b-256 hash (lowercase hex) to
/// `<name>.codehash.txt` in the same directory.
pub fn generate_codehashes_in_output(output_dir: &Path) {
    let Ok(read_dir) = fs::read_dir(output_dir) else {
        return;
    };
    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.extension().map(|e| e == "wasm").unwrap_or(false) {
            let codehash_path = path.with_file_name(format!(
                "{}{}",
                path.file_stem().unwrap().to_string_lossy(),
                CODEHASH_FILE_SUFFIX
            ));
            generate_codehash(&path, &codehash_path);
        }
    }
}
