use super::OutputContract;
use cargo_toml::{Manifest, Value};

pub type CargoTomlContents = Manifest<Value>;

impl OutputContract {
    fn load_cargo_toml_contents(&mut self) -> CargoTomlContents {
        CargoTomlContents::from_path_with_metadata(self.cargo_toml_path()).unwrap()
    }

    pub fn with_cargo_toml_contents<R, F: FnOnce(&CargoTomlContents) -> R>(&mut self, f: F) -> R {
        if let Some(cached_contents) = &self.cargo_toml_contents_cache {
            f(cached_contents)
        } else {
            let contents = self.load_cargo_toml_contents();
            let result = f(&contents);
            self.cargo_toml_contents_cache = Some(contents);
            result
        }
    }

    /// The name of the wasm crate, as defined in its corresponding `Cargo.toml`.
    ///
    /// Note this does not necessarily have to match the name of the crate directory.
    ///
    /// Mutable reference required because there is a config cache.
    pub fn wasm_crate_name(&mut self) -> String {
        self.with_cargo_toml_contents(|contents| {
            contents
                .package
                .as_ref()
                .expect("wasm crate Cargo.toml is missing a package config")
                .name
                .clone()
        })
    }

    pub fn wasm_crate_name_snake_case(&mut self) -> String {
        self.wasm_crate_name().replace('-', "_")
    }

    /// Can be used to edit and save a Cargo.toml file.
    ///
    /// Not currently used to manage Cargo.toml contents, but could be.
    ///
    /// Warning! Not very reliable. Its output differs from its source. Most notably it writes "rlib" instead of "cdylib".
    pub fn reserialize_cargo_toml(&mut self) -> String {
        self.with_cargo_toml_contents(|manifest| {
            toml::to_string(&manifest).expect("Could not encode TOML value")
        })
    }
}
