use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use serde::Serialize;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use multiversx_sc_meta_lib::cargo_toml::CargoTomlContents;

use crate::cli::PackArgs;
use crate::folder_structure::RelevantDirectories;

use super::local_deps::{DependencyDepth, compute_local_deps};
use super::source_json_model::{
    PackedSource, SCHEMA_VERSION, SOURCE_JSON_EXTENSION, SYS_MAXSIZE, SourceBuildMetadata,
    SourceBuildOptions, SourceFileEntry, SourceMetadata,
};

/// File names (regardless of extension) that are included as source files.
const NAMED_SOURCE_FILES: &[&str] = &[
    "Cargo.toml",
    "Cargo.lock",
    "multicontract.toml",
    "sc-config.toml",
    "multiversx.json",
];

/// Packages the source code for all contracts found in `project_folder`.
///
/// `project_folder` can be a workspace root (containing multiple contracts) or a
/// single contract folder. Mirrors the behaviour of the Python builder's
/// `build_project` / `create_packaged_source_code` functions.
///
/// For each contract, writes:
///   `<contract_dir>/output/<name>-<version>.source.json`
pub fn source_pack(args: &PackArgs) {
    let project_folder = if let Some(p) = &args.path {
        Path::new(p).canonicalize().unwrap()
    } else {
        Path::new(".").canonicalize().unwrap()
    };

    let dirs = RelevantDirectories::find_all(&project_folder, &["target".to_string()]);
    if dirs.iter_contract_crates().count() == 0 {
        println!(
            "No contracts found (no multiversx.json) under: {}",
            project_folder.display()
        );
        return;
    }

    for dir in dirs.iter_contract_crates() {
        source_pack_contract(&project_folder, &dir.path, args.contract.as_deref());
    }
}

/// Packs source for one contract, with all paths relative to `project_folder`.
/// If `specific_contract` is `Some`, skips contracts whose name doesn't match.
pub(crate) fn source_pack_contract(
    project_folder: &Path,
    contract_folder: &Path,
    specific_contract: Option<&str>,
) {
    let cargo_toml = CargoTomlContents::load_from_file(contract_folder.join("Cargo.toml"));
    let contract_name = cargo_toml.package_name();
    if let Some(filter) = specific_contract {
        if contract_name != filter {
            return;
        }
    }
    let contract_version = cargo_toml.package_version();

    let local_deps = compute_local_deps(contract_folder);

    let mut entries: Vec<SourceFileEntry> = Vec::new();
    let mut added: HashSet<PathBuf> = HashSet::new();

    // 1. Files from the contract folder itself (depth 0, module = contract relative to project)
    let contract_module = module_path(project_folder, contract_folder);
    for file in collect_source_files(contract_folder) {
        entries.push(make_entry(&file, project_folder, &contract_module, 0u64));
        added.insert(file);
    }

    // 2. Files from each local dependency folder
    for dep in &local_deps.dependencies {
        let dep_folder = contract_folder.join(&dep.path).canonicalize().unwrap();
        let dep_module = module_path(project_folder, &dep_folder);
        for file in collect_source_files(&dep_folder) {
            if added.contains(&file) {
                continue;
            }
            entries.push(make_entry(&file, project_folder, &dep_module, dep.depth));
            added.insert(file);
        }
    }

    // 3. Remaining files from the project folder (catches workspace-level Cargo.lock, etc.)
    //    This is a no-op when project_folder == contract_folder.
    for file in collect_source_files(project_folder) {
        if !added.contains(&file) {
            entries.push(make_entry(
                &file,
                project_folder,
                &contract_module,
                SYS_MAXSIZE,
            ));
            added.insert(file);
        }
    }

    // Sort by (dependency_depth, path) to match the Python builder output
    entries.sort_by(|a, b| {
        a.dependency_depth
            .cmp(&b.dependency_depth)
            .then(a.path.cmp(&b.path))
    });

    let packed = PackedSource {
        schema_version: SCHEMA_VERSION.to_string(),
        metadata: SourceMetadata {
            contract_name: contract_name.clone(),
            contract_version: contract_version.clone(),
            build_metadata: SourceBuildMetadata::default(),
            build_options: SourceBuildOptions {
                package_whole_project_src: true,
                specific_contract: specific_contract.map(|s| s.to_string()),
                build_root_folder: project_folder.to_string_lossy().into_owned(),
            },
        },
        entries,
    };

    let output_dir = contract_folder.join("output");
    fs::create_dir_all(&output_dir).unwrap();
    let output_path = output_dir.join(format!(
        "{contract_name}-{contract_version}{SOURCE_JSON_EXTENSION}"
    ));

    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut buf = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    packed.serialize(&mut ser).unwrap();
    buf.push(b'\n');
    fs::write(&output_path, &buf).unwrap();

    println!("Source packed to: {}", output_path.display());
}

/// Returns the path of `folder` relative to `project_folder`, using forward slashes.
fn module_path(project_folder: &Path, folder: &Path) -> String {
    pathdiff::diff_paths(folder, project_folder)
        .unwrap_or_else(|| folder.to_path_buf())
        .to_string_lossy()
        .replace('\\', "/")
}

fn make_entry(
    file: &Path,
    project_folder: &Path,
    module: &str,
    depth: DependencyDepth,
) -> SourceFileEntry {
    let rel = pathdiff::diff_paths(file, project_folder).unwrap();
    let path_str = rel.to_string_lossy().replace('\\', "/");
    let content = BASE64.encode(fs::read(file).unwrap());
    SourceFileEntry {
        path: path_str.clone(),
        content,
        module: module.to_string(),
        dependency_depth: depth,
        is_test_file: is_test_file(&path_str),
    }
}

/// A file is a test file if it is a `.rs` file whose path contains a `test` or `tests` component.
fn is_test_file(path_str: &str) -> bool {
    if !path_str.ends_with(".rs") {
        return false;
    }
    path_str
        .split('/')
        .any(|component| component == "test" || component == "tests")
}

fn collect_source_files(folder: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    collect_recursive(folder, &mut result);
    result.sort();
    result
}

fn collect_recursive(current: &Path, result: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(current) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().map(|n| n != "target").unwrap_or(true) {
                collect_recursive(&path, result);
            }
        } else if is_source_file(&path) {
            result.push(path);
        }
    }
}

fn is_source_file(path: &Path) -> bool {
    if path.extension().map(|e| e == "rs").unwrap_or(false) {
        return true;
    }
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        NAMED_SOURCE_FILES.contains(&name)
    } else {
        false
    }
}
