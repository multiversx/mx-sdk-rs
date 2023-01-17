use std::{fs, path::Path};

use colored::Colorize;
use ruplacer::{Console, DirectoryPatcher, Query, Settings};
use toml::Value;

use crate::CargoTomlContents;

use super::{upgrade_versions::FRAMEWORK_CRATE_NAMES, version_req::VersionReq};

/// Uses ruplacer.
pub(crate) fn replace_in_files(sc_crate_path: &Path, file_type: &str, queries: &[Query]) {
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

/// Uses `CargoTomlContents`. Will only replace versions of framework crates.
pub fn version_bump_in_cargo_toml(path: &Path, from_version: &str, to_version: &str) {
    if path.is_file() {
        if let Some(file_name) = path.file_name() {
            if file_name == "Cargo.toml" {
                let mut cargo_toml_contents = CargoTomlContents::load_from_file(path);
                upgrade_dependencies_version(
                    &mut cargo_toml_contents,
                    "dependencies",
                    from_version,
                    to_version,
                );
                upgrade_dependencies_version(
                    &mut cargo_toml_contents,
                    "dev-dependencies",
                    from_version,
                    to_version,
                );
                cargo_toml_contents.save_to_file(path);
                return;
            }
        }
    }

    if path.is_dir() {
        let read_dir = fs::read_dir(path).expect("error reading directory");
        for child_result in read_dir {
            let child = child_result.unwrap();
            version_bump_in_cargo_toml(child.path().as_path(), from_version, to_version);
        }
    }
}

fn upgrade_dependencies_version(
    cargo_toml_contents: &mut CargoTomlContents,
    deps_name: &str,
    from_version: &str,
    to_version: &str,
) {
    if let Some(deps) = cargo_toml_contents.toml_value.get_mut(deps_name) {
        for &framework_crate_name in FRAMEWORK_CRATE_NAMES {
            if let Some(dep) = deps.get_mut(framework_crate_name) {
                match dep {
                    Value::String(version_string) => {
                        change_version_string(
                            version_string,
                            from_version,
                            to_version,
                            &cargo_toml_contents.path,
                            deps_name,
                            framework_crate_name,
                        );
                    },
                    Value::Table(t) => {
                        if let Some(Value::String(version_string)) = t.get_mut("version") {
                            change_version_string(
                                version_string,
                                from_version,
                                to_version,
                                &cargo_toml_contents.path,
                                deps_name,
                                framework_crate_name,
                            );
                        }
                    },
                    _ => {},
                }
            }
        }
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

fn print_version_change(
    path: &Path,
    deps_name: &str,
    framework_crate_name: &str,
    from: &str,
    to: &str,
) {
    println!(
        "{}/{}/{}: {} -> {}",
        path.display(),
        deps_name,
        framework_crate_name.underline(),
        format!("\"{from}\"").red().strikethrough(),
        format!("\"{to}\"").green()
    )
}
