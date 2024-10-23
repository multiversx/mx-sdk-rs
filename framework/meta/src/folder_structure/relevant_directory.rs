use crate::version::FrameworkVersion;
use multiversx_sc_meta_lib::cargo_toml::{CargoTomlContents, DependencyReference};
use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

// use super::{version_req::VersionReq, DependencyReference, GitReference};

/// Used for retrieving crate versions.
pub const FRAMEWORK_CRATE_NAMES: &[&str] = &[
    "multiversx-sc",
    "multiversx-sc-meta",
    "multiversx-sc-meta-lib",
    "multiversx-sc-scenario",
    "multiversx-sc-snippets",
    "multiversx-sc-wasm-adapter",
    "multiversx-sc-modules",
    "elrond-wasm",
    "elrond-wasm-debug",
    "elrond-wasm-modules",
    "elrond-wasm-node",
    "elrond-interact-snippets",
];

pub const CARGO_TOML_FILE_NAME: &str = "Cargo.toml";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectoryType {
    Contract,
    Lib,
}

#[derive(Debug, Clone)]
pub struct RelevantDirectory {
    pub path: PathBuf,
    pub version: DependencyReference,
    pub upgrade_in_progress: Option<(FrameworkVersion, FrameworkVersion)>,
    pub dir_type: DirectoryType,
}

impl RelevantDirectory {
    pub fn dir_name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }

    pub fn dir_name_underscores(&self) -> String {
        self.dir_name().replace('-', "_")
    }
}

pub struct RelevantDirectories(pub(crate) Vec<RelevantDirectory>);

impl RelevantDirectories {
    pub fn find_all(path: impl AsRef<Path>, ignore: &[String]) -> Self {
        let path_ref = path.as_ref();
        let canonicalized = fs::canonicalize(path_ref).unwrap_or_else(|err| {
            panic!(
                "error canonicalizing input path {}: {}",
                path_ref.display(),
                err,
            )
        });
        let mut dirs = Vec::new();
        populate_directories(canonicalized.as_path(), ignore, &mut dirs);
        RelevantDirectories(dirs)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = &RelevantDirectory> {
        self.0.iter()
    }

    pub fn iter_contract_crates(&self) -> impl Iterator<Item = &RelevantDirectory> {
        self.0
            .iter()
            .filter(|dir| dir.dir_type == DirectoryType::Contract)
    }

    pub fn count_for_version(&self, version: &FrameworkVersion) -> usize {
        self.0
            .iter()
            .filter(|dir| dir.version.is_framework_version(version))
            .count()
    }

    pub fn iter_version(
        &mut self,
        version: &'static FrameworkVersion,
    ) -> impl Iterator<Item = &RelevantDirectory> {
        self.0
            .iter()
            .filter(move |dir| dir.version.is_framework_version(version))
    }

    /// Marks all appropriate directories as ready for upgrade.
    pub fn start_upgrade(&mut self, from_version: FrameworkVersion, to_version: FrameworkVersion) {
        for dir in self.0.iter_mut() {
            if dir.version.is_framework_version(&from_version) {
                dir.upgrade_in_progress = Some((from_version.clone(), to_version.clone()));
            }
        }
    }

    /// Updates the versions of all directories being upgraded (in memory)
    /// and resets upgrade status.
    pub fn finish_upgrade(&mut self) {
        for dir in self.0.iter_mut() {
            if let Some((_, to_version)) = &dir.upgrade_in_progress {
                if let DependencyReference::Version(version_req) = &mut dir.version {
                    version_req.semver = to_version.clone();
                }
                dir.upgrade_in_progress = None;
            }
        }
    }
}

fn populate_directories(path: &Path, ignore: &[String], result: &mut Vec<RelevantDirectory>) {
    let is_contract = is_marked_contract_crate_dir(path);

    if !is_contract && path.is_dir() {
        let read_dir = fs::read_dir(path).expect("error reading directory");
        for child_result in read_dir {
            let child = child_result.unwrap();
            if can_continue_recursion(&child, ignore) {
                populate_directories(child.path().as_path(), ignore, result);
            }
        }
    }

    if let Some(version) = find_framework_dependency(path) {
        let dir_type = if is_contract {
            DirectoryType::Contract
        } else {
            DirectoryType::Lib
        };
        result.push(RelevantDirectory {
            path: path.to_owned(),
            version,
            upgrade_in_progress: None,
            dir_type,
        });
    }
}

fn is_marked_contract_crate_dir(path: &Path) -> bool {
    path.join("multiversx.json").is_file() || path.join("elrond.json").is_file()
}

fn can_continue_recursion(dir_entry: &DirEntry, blacklist: &[String]) -> bool {
    if !dir_entry.file_type().unwrap().is_dir() {
        return false;
    }

    if let Some(dir_name_str) = dir_entry.file_name().to_str() {
        if blacklist.iter().any(|ignored| ignored == dir_name_str) {
            return false;
        }

        // do not explore hidden folders
        !dir_name_str.starts_with('.')
    } else {
        false
    }
}

fn load_cargo_toml_contents(dir_path: &Path) -> Option<CargoTomlContents> {
    let cargo_toml_path = dir_path.join(CARGO_TOML_FILE_NAME);
    if cargo_toml_path.is_file() {
        Some(CargoTomlContents::load_from_file(cargo_toml_path))
    } else {
        None
    }
}

impl RelevantDirectory {
    #[allow(unused)]
    pub fn cargo_toml_contents(&self) -> Option<CargoTomlContents> {
        load_cargo_toml_contents(self.path.as_path())
    }
}

fn find_framework_dependency(dir_path: &Path) -> Option<DependencyReference> {
    if let Some(cargo_toml_contents) = load_cargo_toml_contents(dir_path) {
        for &crate_name in FRAMEWORK_CRATE_NAMES {
            if let Some(dep_raw) = cargo_toml_contents.dependency_raw_value(crate_name) {
                return Some(dep_raw.interpret());
            }
        }
    }

    None
}
