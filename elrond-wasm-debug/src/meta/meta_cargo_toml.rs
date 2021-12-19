use std::fs::{self, create_dir_all};

// use cargo_toml::{Manifest, Value};

use super::meta_config::{ContractMetadata, MetaConfig};

impl MetaConfig {
    pub fn create_wasm_view_cargo_toml(&self) {
        if let Some(main_contract) = &self.main_contract {
            if let Some(view_contract) = &self.view_contract {
                create_dir_all(&view_contract.wasm_crate_path).unwrap();
                create_cargo_toml_from_source(main_contract, view_contract);
            }
        }
    }
}

fn create_cargo_toml_from_source(source: &ContractMetadata, dest: &ContractMetadata) {
    // TODO: find a clean & elegant way of changing the crate name
    // Below was an attempt to parse it as Rust Manifest, but the result was a mess
    // It also works fine without changing the crate name, so this is not urgent

    // let mut manifest = Manifest::<Value>::from_path_with_metadata(source).unwrap();
    // if let Some(package) = &mut manifest.package {
    //     package.name = view_contract.wasm_crate_name.clone();
    // }
    // let mut wasm_view_cargo_file = File::create(dest).unwrap();
    // let toml_string = toml::to_string(&manifest).expect("Could not encode TOML value");
    // write!(wasm_view_cargo_file, "{}", toml_string).unwrap();

    fs::copy(source.cargo_toml_path(), dest.cargo_toml_path()).unwrap();
}
