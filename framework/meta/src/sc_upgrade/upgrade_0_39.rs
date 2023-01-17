use std::path::Path;

use crate::CargoTomlContents;
use ruplacer::Query;
use toml::{value::Table, Value};

use super::{
    folder_structure::{DirectoryToUpdate, DirectoryType},
    upgrade_common::replace_in_files,
};

pub(crate) fn upgrade_39(dir: &DirectoryToUpdate) {
    if dir.dir_type == DirectoryType::Contract {
        v_0_39_prepare_meta(&dir.path);
        v_0_39_prepare_wasm(&dir.path);
    }
    v_0_39_replace_in_files(&dir.path);
}

fn v_0_39_prepare_meta(sc_crate_path: &Path) {
    let cargo_toml_path = sc_crate_path.clone().join("meta/Cargo.toml");
    assert!(
        cargo_toml_path.exists(),
        "SC crate Cargo.toml not found: {}",
        cargo_toml_path.display()
    );
    let mut meta_cargo_toml = CargoTomlContents::load_from_file(&cargo_toml_path);
    let deps = meta_cargo_toml.dependencies_mut();

    println!("Fixing meta crate");
    deps.remove("elrond-wasm");
    deps.remove("elrond-wasm-debug");

    let mut meta_dep = Table::new();
    meta_dep.insert("version".to_string(), Value::String("0.39.0".to_string()));
    deps.insert("multiversx-sc-meta".to_string(), Value::Table(meta_dep));

    meta_cargo_toml.save_to_file(&cargo_toml_path);
}

fn v_0_39_prepare_wasm(sc_crate_path: &Path) {
    let cargo_toml_path = sc_crate_path.clone().join("wasm/Cargo.toml");
    assert!(
        cargo_toml_path.exists(),
        "SC crate Cargo.toml not found: {}",
        cargo_toml_path.display()
    );
    let mut meta_cargo_toml = CargoTomlContents::load_from_file(&cargo_toml_path);
    let deps = meta_cargo_toml.dependencies_mut();

    println!("Removing elrond-wasm-output");
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
        ][..],
    );
}
