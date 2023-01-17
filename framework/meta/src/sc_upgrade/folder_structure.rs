use crate::CargoTomlContents;
use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};
use toml::Value;

/// Used for retrieving  crate versions.
const FRAMEWORK_CRATE_NAMES: &[&str] = &[
    "multiversx-sc",
    "multiversx-sc-scenarios",
    "multiversx-sc-meta",
    "multiversx-sc-modules",
    "elrond-wasm",
    "elrond-wasm-debug",
    "elrond-wasm-modules",
    "elrond-wasm-node",
    "elrond-interact-snippets",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectoryType {
    Contract,
    Lib,
}

#[derive(Debug, Clone)]
pub struct VersionSpecification {
    pub semver: String,
    pub is_strict: bool,
}

#[derive(Debug, Clone)]
pub struct DirectoryToUpdate {
    pub path: PathBuf,
    pub version: VersionSpecification,
    pub dir_type: DirectoryType,
}

pub(crate) fn populate_directories(path: &Path, result: &mut Vec<DirectoryToUpdate>) {
    let is_contract = is_marked_contract_crate_dir(path);

    if !is_contract && path.is_dir() {
        let read_dir = fs::read_dir(path).expect("error reading directory");
        for child_result in read_dir {
            let child = child_result.unwrap();
            // println!("{}", child.path().display());

            if continue_recursion(&child) {
                populate_directories(child.path().as_path(), result);
            }
        }
    }

    if let Some(version) = find_framework_version(path) {
        let dir_type = if is_contract {
            DirectoryType::Contract
        } else {
            DirectoryType::Lib
        };
        result.push(DirectoryToUpdate {
            path: path.to_owned(),
            version,
            dir_type,
        });
    }
}

fn is_marked_contract_crate_dir(path: &Path) -> bool {
    path.join("multiversx.json").is_file() || path.join("elrond.json").is_file()
}

fn continue_recursion(dir_entry: &DirEntry) -> bool {
    if !dir_entry.file_type().unwrap().is_dir() {
        return false;
    }

    if let Some(dir_name_str) = dir_entry.file_name().to_str() {
        // not hidden
        !dir_name_str.starts_with('.')
    } else {
        false
    }
}

fn find_framework_version_string(cargo_toml_contents: &CargoTomlContents) -> Option<String> {
    for &crate_name in FRAMEWORK_CRATE_NAMES {
        if let Some(old_base) = cargo_toml_contents.dependency(crate_name) {
            if let Some(Value::String(s)) = old_base.get("version") {
                return Some(s.clone());
            }
        }
    }

    None
}

impl VersionSpecification {
    fn from_string(raw: String) -> Self {
        if let Some(stripped_version) = raw.strip_prefix('=') {
            VersionSpecification {
                semver: stripped_version.to_string(),
                is_strict: true,
            }
        } else {
            VersionSpecification {
                semver: raw,
                is_strict: true,
            }
        }
    }
}

fn find_framework_version(dir_path: &Path) -> Option<VersionSpecification> {
    let cargo_toml_path = dir_path.join("Cargo.toml");
    if cargo_toml_path.is_file() {
        let cargo_toml_contents = CargoTomlContents::load_from_file(cargo_toml_path);
        if let Some(version) = find_framework_version_string(&cargo_toml_contents) {
            return Some(VersionSpecification::from_string(version));
        }
    }

    None
}
