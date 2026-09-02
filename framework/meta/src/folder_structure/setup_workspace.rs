use multiversx_sc_meta_lib::cargo_toml::CargoTomlContents;
use std::{
    fs::{self, DirEntry},
    path::Path,
};

use super::CARGO_TOML_FILE_NAME;

/// Adds a `[workspace]` section to the `Cargo.toml` at `path`, making the
/// project self-contained (isolated from any surrounding workspace).
///
/// For projects that already have a root `Cargo.toml` (e.g. a single-crate
/// project or a crate that is also the workspace root), the `[workspace]`
/// section is inserted into the existing file.  For multi-crate projects whose
/// root has no `Cargo.toml` yet (a virtual workspace layout), a new manifest
/// is created containing only the `[workspace]` section.
///
/// Workspace members are discovered by recursively scanning `path` for
/// sub-directories that contain a `Cargo.toml`.  Directories listed in
/// `ignore` and hidden directories are skipped, mirroring the behaviour of
/// [`strip_path`](super::strip_path::strip_path).
///
/// The root itself is always added as the `"."` member when it already has a
/// `Cargo.toml`.
pub fn setup_workspace(path: &Path, ignore: &[String]) {
    let canonicalized = fs::canonicalize(path)
        .unwrap_or_else(|err| panic!("error canonicalizing path {}: {}", path.display(), err));

    let members = collect_cargo_dirs(&canonicalized, &canonicalized, ignore);
    let member_refs: Vec<&str> = members.iter().map(String::as_str).collect();

    let root_cargo_toml = canonicalized.join(CARGO_TOML_FILE_NAME);
    let mut contents = if root_cargo_toml.exists() {
        CargoTomlContents::load_from_file(&root_cargo_toml)
    } else {
        CargoTomlContents::new()
    };

    contents.add_workspace(&member_refs);
    contents.save_to_file(&root_cargo_toml);

    println!(
        "Added [workspace] with {} member(s) to {}",
        member_refs.len(),
        root_cargo_toml.display(),
    );
}

/// Recursively collects relative paths (using `/` as separator) for every
/// directory under `current` that contains a `Cargo.toml`.
///
/// The root itself is represented as `"."` when it has a `Cargo.toml`.
/// Entries in `ignore` and hidden directories are skipped.
fn collect_cargo_dirs(base: &Path, current: &Path, ignore: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    collect_recursive(base, current, ignore, &mut result);
    result.sort();
    result
}

fn collect_recursive(base: &Path, current: &Path, ignore: &[String], result: &mut Vec<String>) {
    if current.join(CARGO_TOML_FILE_NAME).is_file() {
        let rel = current.strip_prefix(base).unwrap();
        result.push(if rel.as_os_str().is_empty() {
            ".".to_string()
        } else {
            rel.to_str()
                .unwrap()
                .replace(std::path::MAIN_SEPARATOR, "/")
        });
    }

    if let Ok(entries) = fs::read_dir(current) {
        let mut children: Vec<DirEntry> = entries.flatten().collect();
        children.sort_by_key(|e| e.file_name());
        for child in children {
            if can_recurse_into(&child, ignore) {
                collect_recursive(base, &child.path(), ignore, result);
            }
        }
    }
}

fn can_recurse_into(entry: &DirEntry, ignore: &[String]) -> bool {
    if !entry.file_type().is_ok_and(|ft| ft.is_dir()) {
        return false;
    }
    if let Some(name) = entry.file_name().to_str() {
        if ignore.iter().any(|ignored| ignored == name) {
            return false;
        }
        !name.starts_with('.')
    } else {
        false
    }
}
