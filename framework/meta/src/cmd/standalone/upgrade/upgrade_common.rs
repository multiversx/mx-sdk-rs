use ruplacer::{Console, DirectoryPatcher, Query, Settings};
use std::{
    fs,
    path::Path,
    process::{self, Command},
};
use toml::Value;

use crate::{
    cargo_toml_contents::{CARGO_TOML_DEPENDENCIES, CARGO_TOML_DEV_DEPENDENCIES},
    cmd::standalone::all::call_contract_meta,
    folder_structure::{
        DirectoryType, RelevantDirectory, VersionReq, CARGO_TOML_FILE_NAME, FRAMEWORK_CRATE_NAMES,
    },
    CargoTomlContents,
};

use super::upgrade_print::*;

/// Uses ruplacer.
pub(crate) fn replace_in_files(sc_crate_path: &Path, file_type: &str, queries: &[Query]) {
    if !sc_crate_path.exists() {
        return;
    }

    let console = Console::default();
    let settings = Settings {
        selected_file_types: vec![file_type.to_string()],
        ..Default::default()
    };
    let mut directory_patcher = DirectoryPatcher::new(&console, sc_crate_path, &settings);
    for query in queries {
        directory_patcher.run(query).expect("replace failed");
    }
}

/// Regex was not needed yet, add if it becomes necessary.
pub(crate) fn rename_files(path: &Path, patterns: &[(&str, &str)]) {
    if let Some(file_name_str) = try_get_file_name_str(path) {
        if let Some(replaced_file_name) = try_replace_file_name(file_name_str, patterns) {
            let replaced_path = path.parent().unwrap().join(replaced_file_name);
            print_rename(path, replaced_path.as_path());
            fs::rename(path, replaced_path).expect("failed to rename file");
        }
    }

    if path.is_dir() {
        let read_dir = fs::read_dir(path).expect("error reading directory");
        for child_result in read_dir {
            let child = child_result.unwrap();
            rename_files(child.path().as_path(), patterns);
        }
    }
}

fn try_get_file_name_str(path: &Path) -> Option<&str> {
    if !path.is_file() {
        return None;
    }
    if let Some(file_name) = path.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            return Some(file_name_str);
        }
    }
    None
}

fn try_replace_file_name(file_name_str: &str, patterns: &[(&str, &str)]) -> Option<String> {
    for &(replace_pattern, replace_with) in patterns {
        if file_name_str.contains(replace_pattern) {
            return Some(file_name_str.replace(replace_pattern, replace_with));
        }
    }
    None
}

/// Uses `CargoTomlContents`. Will only replace versions of framework crates.
pub fn version_bump_in_cargo_toml(path: &Path, from_version: &str, to_version: &str) {
    if is_cargo_toml_file(path) {
        let mut cargo_toml_contents = CargoTomlContents::load_from_file(path);
        upgrade_all_dependency_versions(
            &mut cargo_toml_contents,
            CARGO_TOML_DEPENDENCIES,
            from_version,
            to_version,
        );
        upgrade_all_dependency_versions(
            &mut cargo_toml_contents,
            CARGO_TOML_DEV_DEPENDENCIES,
            from_version,
            to_version,
        );
        cargo_toml_contents.save_to_file(path);
        return;
    }

    if path.is_dir() {
        let read_dir = fs::read_dir(path).expect("error reading directory");
        for child_result in read_dir {
            let child = child_result.unwrap();
            version_bump_in_cargo_toml(child.path().as_path(), from_version, to_version);
        }
    }
}

fn is_cargo_toml_file(path: &Path) -> bool {
    if let Some(file_name_str) = try_get_file_name_str(path) {
        file_name_str == CARGO_TOML_FILE_NAME
    } else {
        false
    }
}

fn upgrade_all_dependency_versions(
    cargo_toml_contents: &mut CargoTomlContents,
    deps_name: &str,
    from_version: &str,
    to_version: &str,
) {
    if let Some(dependencies) = cargo_toml_contents.toml_value.get_mut(deps_name) {
        for &framework_crate_name in FRAMEWORK_CRATE_NAMES {
            upgrade_dependency_version(
                &cargo_toml_contents.path,
                deps_name,
                dependencies,
                framework_crate_name,
                from_version,
                to_version,
            );
        }
    }
}

fn upgrade_dependency_version(
    cargo_toml_path: &Path,
    deps_name: &str,
    dependencies: &mut Value,
    framework_crate_name: &str,
    from_version: &str,
    to_version: &str,
) {
    match dependencies.get_mut(framework_crate_name) {
        Some(Value::String(version_string)) => {
            change_version_string(
                version_string,
                from_version,
                to_version,
                cargo_toml_path,
                deps_name,
                framework_crate_name,
            );
        },
        Some(Value::Table(t)) => {
            if let Some(Value::String(version_string)) = t.get_mut("version") {
                change_version_string(
                    version_string,
                    from_version,
                    to_version,
                    cargo_toml_path,
                    deps_name,
                    framework_crate_name,
                );
            }
        },
        _ => {},
    }
}

fn change_version_string(
    version_string: &mut String,
    from_version: &str,
    to_version: &str,
    path: &Path,
    deps_name: &str,
    framework_crate_name: &str,
) {
    let version_string_before = version_string.clone();
    let mut version_spec = VersionReq::from_string(std::mem::take(version_string));
    if version_spec.semver == from_version {
        version_spec.semver = to_version.to_string();
    }
    *version_string = version_spec.into_string();

    print_version_change(
        path,
        deps_name,
        framework_crate_name,
        version_string_before.as_str(),
        version_string.as_str(),
    );
}

pub fn re_generate_wasm_crate(dir: &RelevantDirectory) {
    if dir.dir_type != DirectoryType::Contract {
        return;
    }
    call_contract_meta(
        &dir.path,
        &["abi".to_string(), "--no-abi-git-version".to_string()],
    );
}

pub fn cargo_check(dir: &RelevantDirectory) {
    print_cargo_check(dir);

    let result = Command::new("cargo")
        .current_dir(&dir.path)
        .args(["check", "--tests"])
        .spawn()
        .expect("failed to spawn cargo check process")
        .wait()
        .expect("cargo check was not running");

    if !result.success() {
        print_cargo_check_fail();
        process::exit(1);
    }
}
