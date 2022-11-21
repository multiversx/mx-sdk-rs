use std::fs::{self, create_dir_all};

// use cargo_toml::{Manifest, Value};

use super::{meta_config::MetaConfig, output_contract::OutputContract};

impl MetaConfig {
    pub fn create_wasm_view_cargo_toml(&self) {
        let main_contract = self.output_contracts.main_contract();
        for secondary_contract in self.output_contracts.secondary_contracts() {
            create_dir_all(&secondary_contract.wasm_crate_path()).unwrap();
            create_cargo_toml_from_source(main_contract, secondary_contract);
        }
    }
}

fn create_cargo_toml_from_source(source: &OutputContract, dest: &OutputContract) {
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
