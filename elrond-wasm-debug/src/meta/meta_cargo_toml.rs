use std::fs::{self, create_dir_all};

// use cargo_toml::{Manifest, Value};

use super::meta_config::MetaConfig;

impl MetaConfig {
    pub fn create_wasm_view_cargo_toml(&self) {
        if let Some(main_contract) = &self.main_contract {
            if let Some(view_contract) = &self.view_contract {
                let source = format!("{}/Cargo.toml", &main_contract.wasm_crate_path);
                // let mut manifest = Manifest::<Value>::from_path_with_metadata(source).unwrap();
                // if let Some(package) = &mut manifest.package {
                //     package.name = view_contract.wasm_crate_name.clone();
                // }

                create_dir_all(&view_contract.wasm_crate_path).unwrap();
                let dest = format!("{}/Cargo.toml", &view_contract.wasm_crate_path);
                // let mut wasm_view_cargo_file = File::create(dest).unwrap();
                // let toml_string = toml::to_string(&manifest).expect("Could not encode TOML value");
                // write!(wasm_view_cargo_file, "{}", toml_string).unwrap();
                fs::copy(source, dest).unwrap();
            }
        }
    }
}
