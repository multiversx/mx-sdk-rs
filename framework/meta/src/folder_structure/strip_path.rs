use super::{DirectoryType, FRAMEWORK_CRATE_NAMES, RelevantDirectories, RelevantDirectory};
use multiversx_sc_meta_lib::cargo_toml::{CargoTomlContents, WorkspaceDependencies};
use std::path::Path;

/// Recursively finds all crates under `path` that depend on a framework crate,
/// resolves any `workspace = true` entries using `workspace_dependencies`, then
/// removes `path = "..."` from those framework dependencies in each
/// `Cargo.toml`. Directories named in `ignore` are skipped during traversal.
/// For contract crates, the `meta/` subdirectory is also processed.
pub fn strip_path(path: &Path, ignore: &[String], workspace_dependencies: &WorkspaceDependencies) {
    let dirs = RelevantDirectories::find_all(path, ignore);

    let mut count = 0;
    for dir in dirs.iter() {
        if strip_framework_paths(dir, workspace_dependencies) {
            count += 1;
        }
        // `populate_directories` stops recursing into a contract crate's children,
        // so the meta sub-crate is never included in `dirs` — handle it explicitly.
        if dir.dir_type == DirectoryType::Contract {
            let meta_dir = RelevantDirectory {
                path: dir.meta_path(),
                ..dir.clone()
            };
            if strip_framework_paths(&meta_dir, workspace_dependencies) {
                count += 1;
            }
        }
    }

    println!("Done. Stripped path from {count} Cargo.toml file(s).");
}

/// Resolves `workspace = true` entries, then removes `path = "..."` from
/// framework dependencies in a directory's Cargo.toml.
/// Returns `true` if the file was modified.
fn strip_framework_paths(
    dir: &RelevantDirectory,
    workspace_dependencies: &WorkspaceDependencies,
) -> bool {
    let cargo_toml_path = dir.path.join("Cargo.toml");
    if !cargo_toml_path.is_file() {
        return false;
    }

    let mut cargo_toml = CargoTomlContents::load_from_file(&cargo_toml_path);
    let ws_changed = cargo_toml.resolve_workspace_dependencies(workspace_dependencies);
    let paths_stripped = cargo_toml.strip_dependency_paths(FRAMEWORK_CRATE_NAMES);

    if paths_stripped {
        println!("Stripped: {}", cargo_toml_path.display());
    }
    if ws_changed || paths_stripped {
        cargo_toml.save_to_file(&cargo_toml_path);
    }

    paths_stripped
}
