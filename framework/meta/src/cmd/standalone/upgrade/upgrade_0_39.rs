use std::path::Path;

use super::{
    upgrade_common::{
        re_generate_wasm_crate, rename_files, replace_in_files, version_bump_in_cargo_toml,
    },
    upgrade_print::*,
};
use crate::{
    folder_structure::{DirectoryType, RelevantDirectory},
    CargoTomlContents,
};
use ruplacer::Query;
use toml::{value::Table, Value};

#[rustfmt::skip]
pub const SCENARIO_FILE_PATTERNS: &[(&str, &str)] = &[
    ("mandos_go", "scenario_go"), 
    ("mandos_rs", "scenario_rs"),
];

/// Migrate `0.38.0` to `0.39.0`, including the version bump.
pub fn upgrade_to_39_0(dir: &RelevantDirectory) {
    if dir.dir_type == DirectoryType::Contract {
        v_0_39_prepare_meta(&dir.path);
        v_0_39_prepare_wasm(&dir.path);
    }
    v_0_39_replace_in_files(&dir.path);
    rename_files(dir.path.as_ref(), SCENARIO_FILE_PATTERNS);

    let (from_version, to_version) = dir.upgrade_in_progress.unwrap();
    version_bump_in_cargo_toml(&dir.path, from_version, to_version);
}

/// Post-processing: re-generate the wasm crates.
pub fn postprocessing_after_39_0(dir: &RelevantDirectory) {
    if dir.dir_type != DirectoryType::Contract {
        return;
    }
    print_postprocessing_after_39_1(dir.path.as_path());
    re_generate_wasm_crate(dir);
}

fn v_0_39_prepare_meta(sc_crate_path: &Path) {
    let cargo_toml_path = sc_crate_path.join("meta/Cargo.toml");
    assert!(
        cargo_toml_path.exists(),
        "SC crate Cargo.toml not found: {}",
        cargo_toml_path.display()
    );
    let mut meta_cargo_toml = CargoTomlContents::load_from_file(&cargo_toml_path);
    let deps = meta_cargo_toml.dependencies_mut();

    print_cargo_dep_remove(cargo_toml_path.as_path(), "elrond-wasm");
    deps.remove("elrond-wasm");

    print_cargo_dep_remove(cargo_toml_path.as_path(), "elrond-wasm-debug");
    deps.remove("elrond-wasm-debug");

    print_cargo_dep_add(cargo_toml_path.as_path(), "multiversx-sc-meta");
    let mut meta_dep = Table::new();
    meta_dep.insert("version".to_string(), Value::String("0.39.0".to_string()));
    deps.insert("multiversx-sc-meta".to_string(), Value::Table(meta_dep));

    meta_cargo_toml.save_to_file(&cargo_toml_path);
}

fn v_0_39_prepare_wasm(sc_crate_path: &Path) {
    let cargo_toml_path = sc_crate_path.join("wasm/Cargo.toml");
    assert!(
        cargo_toml_path.exists(),
        "SC crate Cargo.toml not found: {}",
        cargo_toml_path.display()
    );
    let mut meta_cargo_toml = CargoTomlContents::load_from_file(&cargo_toml_path);
    let deps = meta_cargo_toml.dependencies_mut();

    print_cargo_dep_remove(cargo_toml_path.as_path(), "elrond-wasm-output");
    deps.remove("elrond-wasm-output");

    meta_cargo_toml.save_to_file(&cargo_toml_path);
}

fn v_0_39_replace_in_files(sc_crate_path: &Path) {
    replace_in_files(
        sc_crate_path,
        "*Cargo.toml",
        &[
            Query::substring("elrond-wasm-debug", "multiversx-sc-scenario"),
            Query::substring("elrond-wasm-modules", "multiversx-sc-modules"),
            Query::substring("elrond-wasm-node", "multiversx-sc-wasm-adapter"),
            Query::substring("elrond-wasm", "multiversx-sc"),
        ][..],
    );

    replace_in_files(
        sc_crate_path,
        "*rs",
        &[
            Query::substring("elrond_codec", "codec"),
            Query::substring(
                "elrond_wasm_debug::meta::perform",
                "multiversx_sc_meta::cli_main",
            ),
            Query::substring(
                "elrond_wasm_debug::mandos_go",
                "multiversx_sc_scenario::run_go",
            ),
            Query::substring(
                "elrond_wasm_debug::mandos_rs",
                "multiversx_sc_scenario::run_rs",
            ),
            Query::substring("elrond_wasm_debug", "multiversx_sc_scenario"),
            Query::substring("elrond_wasm_modules", "multiversx_sc_modules"),
            Query::substring("elrond_wasm_node", "multiversx_sc_wasm_adapter"),
            Query::substring("elrond_wasm", "multiversx_sc"),
            Query::substring("BlockchainMock", "ScenarioWorld"),
            Query::substring("testing_framework", "whitebox"),
            Query::substring("tx_mock", "whitebox"),
            Query::substring("register_contract_builder", "register_contract"),
        ][..],
    );
}
