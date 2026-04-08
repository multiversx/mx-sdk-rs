use std::path::PathBuf;

use super::wasm_cargo_toml_data::WasmCargoTomlData;
use crate::cargo_toml::{CargoTomlContents, DependencyRawValue, change_from_base_to_adapter_path};

const WASM_ADAPTER: &str = "multiversx-sc-wasm-adapter";
const CDYLIB_CRATE_TYPE: &str = "cdylib";

pub fn generate_wasm_cargo_toml(
    cargo_toml_data: &WasmCargoTomlData,
    crate_name: &str,
) -> CargoTomlContents {
    let mut new_cargo = CargoTomlContents::new();

    //set cargo toml prepend auto generate status
    new_cargo.prepend_auto_generated_comment = true;

    //add package info
    new_cargo.add_package_info(
        &cargo_toml_data.name,
        "0.0.0".to_string(),
        cargo_toml_data.edition.clone(),
        false,
    );

    //add crate type
    new_cargo.add_crate_type(CDYLIB_CRATE_TYPE);

    //add profile
    new_cargo.add_contract_variant_profile(&cargo_toml_data.profile);

    //add deps
    add_wasm_crate_deps(
        &mut new_cargo,
        crate_name,
        &cargo_toml_data.framework_dependency,
    );

    //check features
    if !cargo_toml_data.contract_features.is_empty() {
        new_cargo.change_features_for_parent_crate_dep(
            cargo_toml_data.contract_features.as_slice(),
            cargo_toml_data.contract_default_features,
        );
    }

    //insert default workspace
    new_cargo.add_workspace(&["."]);

    new_cargo
}

fn add_wasm_crate_deps(
    cargo_toml_contents: &mut CargoTomlContents,
    crate_name: &str,
    framework_dependency: &DependencyRawValue,
) {
    let mut wasm_adapter_dep = framework_dependency.clone();
    if let Some(path) = &mut wasm_adapter_dep.path {
        *path = change_from_base_to_adapter_path(path);
    }

    cargo_toml_contents.insert_dependency_raw_value(
        crate_name,
        DependencyRawValue {
            path: Some(PathBuf::from("..")),
            ..Default::default()
        },
    );

    cargo_toml_contents.insert_dependency_raw_value(WASM_ADAPTER, wasm_adapter_dep);
}
