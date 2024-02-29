use std::path::Path;

use super::upgrade_common::version_bump_in_cargo_toml;
use crate::{
    folder_structure::{DirectoryType, RelevantDirectory},
    CargoTomlContents,
};
use toml::Value;

/// Migrate `0.44.0` to `0.45.0`, including the version bump.
pub fn upgrade_to_45_0(dir: &RelevantDirectory) {
    if dir.dir_type == DirectoryType::Contract {
        v_0_45_prepare_meta(&dir.path);
    }
    let (from_version, to_version) = dir.upgrade_in_progress.unwrap();
    version_bump_in_cargo_toml(&dir.path, from_version, to_version);
}

fn v_0_45_prepare_meta(sc_crate_path: &Path) {
    let cargo_toml_path = sc_crate_path.join("meta/Cargo.toml");
    assert!(
        cargo_toml_path.exists(),
        "SC crate Cargo.toml not found: {}",
        cargo_toml_path.display()
    );
    let mut meta_cargo_toml = CargoTomlContents::load_from_file(&cargo_toml_path);
    let deps = meta_cargo_toml.dependencies_mut();
    if let Some(meta_dep) = deps.get_mut("multiversx-sc-meta") {
        let meta_dep_table = meta_dep
            .as_table_mut()
            .expect("multiversx-sc-meta dependency expected to be given as a table");
        meta_dep_table.remove("default-features");
        meta_dep_table.insert("default-features".to_string(), Value::Boolean(false));
    }
    meta_cargo_toml.save_to_file(&cargo_toml_path);
}
