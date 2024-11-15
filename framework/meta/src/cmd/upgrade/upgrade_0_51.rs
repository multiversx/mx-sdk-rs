use super::upgrade_common::{replace_in_files, version_bump_in_cargo_toml};
use super::upgrade_print::*;
use crate::folder_structure::{DirectoryType, RelevantDirectory};
use multiversx_sc_meta_lib::cargo_toml::CargoTomlContents;
use ruplacer::Query;
use std::path::Path;
use toml::Value;

/// Migrate `0.50` to `0.51.0`, including the version bump.
pub fn upgrade_to_51_0(dir: &RelevantDirectory) {
    if dir.dir_type == DirectoryType::Contract {
        v_0_51_prepare_meta(dir.path.as_ref());
    }
    v_0_51_replace_in_files(dir.path.as_ref());

    let (from_version, to_version) = dir.upgrade_in_progress.clone().unwrap();
    version_bump_in_cargo_toml(&dir.path, &from_version, &to_version);
}

fn v_0_51_replace_in_files(sc_crate_path: &Path) {
    replace_in_files(
        sc_crate_path,
        "*rs",
        &[Query::substring(
            "multiversx_sc_meta",
            "multiversx_sc_meta_lib",
        )][..],
    );
}

fn v_0_51_prepare_meta(sc_crate_path: &Path) {
    let cargo_toml_path = sc_crate_path.join("meta/Cargo.toml");
    assert!(
        cargo_toml_path.exists(),
        "SC crate Cargo.toml not found: {}",
        cargo_toml_path.display()
    );
    let mut meta_cargo_toml = CargoTomlContents::load_from_file(&cargo_toml_path);
    let deps = meta_cargo_toml.dependencies_mut();

    print_cargo_dep_remove(cargo_toml_path.as_path(), "multiversx-sc-meta");
    let mut meta_value = deps
        .remove("multiversx-sc-meta")
        .expect("multiversx-sc-meta dependency not found in meta crate");

    if let Some(Value::String(path)) = meta_value.get_mut("path") {
        path.push_str("-lib");
    }

    print_cargo_dep_add(cargo_toml_path.as_path(), "multiversx-sc-meta");
    deps.insert("multiversx-sc-meta-lib".to_string(), meta_value);

    meta_cargo_toml.save_to_file(&cargo_toml_path);
}
