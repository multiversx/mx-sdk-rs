use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

pub const CODEHASH_FILE_SUFFIX: &str = ".codehash.txt";

/// Computes the Blake2b-256 code hash of the `.wasm` file at `wasm_path`, as lowercase hex.
pub fn compute_codehash(wasm_path: &Path) -> String {
    let wasm_bytes = fs::read(wasm_path)
        .unwrap_or_else(|err| panic!("failed to read wasm file {}: {err}", wasm_path.display()));
    hex::encode(multiversx_sc::chain_core::std::code_hash(&wasm_bytes))
}

/// Computes the Blake2b-256 code hash of the `.wasm` file at `wasm_path`,
/// and writes the lowercase hex string to `codehash_path`.
pub fn generate_codehash(wasm_path: &Path, codehash_path: &Path) {
    let hex_hash = compute_codehash(wasm_path);
    fs::write(codehash_path, hex_hash).unwrap_or_else(|err| {
        panic!(
            "failed to write codehash file {}: {err}",
            codehash_path.display()
        )
    });
}

/// The default codehash file for a `.wasm` file: `<name>.codehash.txt`, in the same directory.
pub fn codehash_path_for(wasm_path: &Path) -> PathBuf {
    wasm_path.with_file_name(format!(
        "{}{}",
        wasm_path.file_stem().unwrap().to_string_lossy(),
        CODEHASH_FILE_SUFFIX
    ))
}

/// Collects `.wasm` files under `path`, sorted.
///
/// If `path` is a file, it is returned as the only entry, provided it is a `.wasm` file.
/// If it is a directory, it is searched (recursively if `recursive` is set), skipping
/// directories whose name is in `ignore`.
///
/// Returns an error if any directory in the search cannot be read. A partial listing is
/// never returned: callers such as `verify` would otherwise pass after checking only the
/// part of the tree that happened to be readable.
pub fn find_wasm_files(path: &Path, recursive: bool, ignore: &[String]) -> Result<Vec<PathBuf>> {
    find_files(path, recursive, ignore, &|p| {
        p.extension().is_some_and(|e| e == "wasm")
    })
}

/// Collects existing `.codehash.txt` files under `path`, sorted.
///
/// If `path` is a `.codehash.txt` file, it is returned as the only entry. If it is a `.wasm`
/// file, its codehash file is returned, if it exists. If it is a directory, it is searched
/// like in [`find_wasm_files`]. Any other explicitly named file yields nothing.
///
/// Returns an error on unreadable directories, like [`find_wasm_files`].
pub fn find_codehash_files(
    path: &Path,
    recursive: bool,
    ignore: &[String],
) -> Result<Vec<PathBuf>> {
    if path.is_file() && path.extension().is_some_and(|e| e == "wasm") {
        let codehash_path = codehash_path_for(path);
        return Ok(if codehash_path.is_file() {
            vec![codehash_path]
        } else {
            Vec::new()
        });
    }
    find_files(path, recursive, ignore, &|p| {
        p.file_name()
            .is_some_and(|n| n.to_string_lossy().ends_with(CODEHASH_FILE_SUFFIX))
    })
}

fn find_files(
    path: &Path,
    recursive: bool,
    ignore: &[String],
    matches: &dyn Fn(&Path) -> bool,
) -> Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    if path.is_file() {
        // an explicitly named file still has to be of the kind we are looking for,
        // otherwise `clear` would happily delete any file handed to `--path`
        if matches(path) {
            result.push(path.to_path_buf());
        }
    } else {
        collect_files(path, recursive, ignore, matches, &mut result)?;
    }
    result.sort();
    Ok(result)
}

fn collect_files(
    dir: &Path,
    recursive: bool,
    ignore: &[String],
    matches: &dyn Fn(&Path) -> bool,
    result: &mut Vec<PathBuf>,
) -> Result<()> {
    let read_dir = fs::read_dir(dir).with_context(|| dir_context(dir))?;
    for entry in read_dir {
        let entry = entry.with_context(|| dir_context(dir))?;
        // `DirEntry::file_type` does not follow symlinks, so links are never traversed
        // or matched, and the search cannot leave the selected tree.
        let file_type = entry.file_type().with_context(|| dir_context(dir))?;
        let path = entry.path();
        if file_type.is_dir() {
            let ignored = path
                .file_name()
                .is_some_and(|name| ignore.iter().any(|i| name.to_string_lossy() == i.as_str()));
            if recursive && !ignored {
                collect_files(&path, recursive, ignore, matches, result)?;
            }
        } else if file_type.is_file() && matches(&path) {
            result.push(path);
        }
    }
    Ok(())
}

fn dir_context(dir: &Path) -> String {
    format!("failed to read directory {}", dir.display())
}

/// Scans `output_dir` for `.wasm` files and generates a codehash file for each one.
///
/// For every `<name>.wasm` found, writes the Blake2b-256 hash (lowercase hex) to
/// `<name>.codehash.txt` in the same directory. A missing `output_dir` is not an error;
/// anything else that prevents the scan panics, like the rest of this module.
pub fn generate_codehashes_in_output(output_dir: &Path) {
    if !output_dir.is_dir() {
        return;
    }
    let wasm_files =
        find_wasm_files(output_dir, false, &[]).unwrap_or_else(|err| panic!("{err:#}"));
    for wasm_path in wasm_files {
        generate_codehash(&wasm_path, &codehash_path_for(&wasm_path));
    }
}
