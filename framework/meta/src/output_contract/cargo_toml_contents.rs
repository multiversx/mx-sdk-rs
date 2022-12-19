use std::{fs, io::Write, path::Path};

/// Contains an in-memory representation of a Cargo.toml file.
///
/// Implementation notes:
///
/// - Currently contains a raw toml tree, but in principle it could also work with a cargo_toml::Manifest.
/// - It keeps an ordered representation, thanks to the `toml` `preserve_order` feature.
#[derive(Clone, Debug)]
pub struct CargoTomlContents {
    toml_value: toml::Value,
}

impl CargoTomlContents {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Self {
        let cargo_toml_content = fs::read(path).expect("failed to open Cargo.toml file");
        let cargo_toml_content_str =
            String::from_utf8(cargo_toml_content).expect("error decoding Cargo.toml utf-8");
        let toml_value = cargo_toml_content_str
            .parse::<toml::Value>()
            .expect("failed to parse Cargo.toml toml format");
        CargoTomlContents { toml_value }
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) {
        let cargo_toml_content_str = self.toml_value.to_string();
        let mut file = std::fs::File::create(path).expect("failed to create Cargo.toml file");
        file.write_all(cargo_toml_content_str.as_bytes())
            .expect("failed to write Cargo.toml contents to file");
    }

    /// Assumes that a package section already exists.
    pub fn change_package_name(&mut self, new_package_name: String) {
        let package = self
            .toml_value
            .get_mut("package")
            .expect("missing package in Cargo.toml");
        package
            .as_table_mut()
            .expect("malformed package in Cargo.toml")
            .insert("name".to_string(), toml::Value::String(new_package_name));
    }
}
