use super::wasm_cargo_toml_data::WasmCargoTomlData;
use crate::{cargo_toml_contents::change_from_base_to_adapter_path, CargoTomlContents};

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
        cargo_toml_data.framework_version.as_str(),
        &cargo_toml_data.framework_path,
    );

    //check features
    if !cargo_toml_data.contract_features.is_empty() {
        new_cargo
            .change_features_for_parent_crate_dep(cargo_toml_data.contract_features.as_slice());
    }

    //insert default workspace
    new_cargo.add_workspace(&["."]);

    new_cargo
}

fn add_wasm_crate_deps(
    cargo_toml_contents: &mut CargoTomlContents,
    crate_name: &str,
    adapter_version: &str,
    adapter_path: &Option<String>,
) {
    let mut crate_deps = toml::map::Map::new();
    crate_deps.insert("path".to_string(), toml::Value::String("..".to_string()));

    let mut adapter_deps = toml::map::Map::new();
    adapter_deps.insert(
        "version".to_string(),
        toml::Value::String(adapter_version.to_string()),
    );

    if adapter_path.is_some() {
        adapter_deps.insert(
            "path".to_string(),
            toml::Value::String(change_from_base_to_adapter_path(
                adapter_path.to_owned().unwrap().as_str(),
            )),
        );
    }

    let mut toml_table_adapter = toml::map::Map::new();
    toml_table_adapter.insert(WASM_ADAPTER.to_string(), toml::Value::Table(adapter_deps));

    let mut toml_table_crate = toml::map::Map::new();
    toml_table_crate.insert(crate_name.to_string(), toml::Value::Table(crate_deps));

    toml_table_crate.extend(toml_table_adapter);

    cargo_toml_contents
        .toml_value
        .as_table_mut()
        .expect("add deps cargo toml error wasm adapter")
        .insert(
            "dependencies".to_string(),
            toml::Value::Table(toml_table_crate),
        );
}
