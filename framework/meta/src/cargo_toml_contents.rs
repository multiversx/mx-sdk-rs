use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use toml::{value::Table, Value};

pub const CARGO_TOML_DEPENDENCIES: &str = "dependencies";
pub const CARGO_TOML_DEV_DEPENDENCIES: &str = "dev-dependencies";

/// Contains an in-memory representation of a Cargo.toml file.
///
/// Implementation notes:
///
/// - Currently contains a raw toml tree, but in principle it could also work with a cargo_toml::Manifest.
/// - It keeps an ordered representation, thanks to the `toml` `preserve_order` feature.
#[derive(Clone, Debug)]
pub struct CargoTomlContents {
    pub path: PathBuf,
    pub toml_value: toml::Value,
}

impl CargoTomlContents {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Self {
        let path_ref = path.as_ref();
        let cargo_toml_content = fs::read(path_ref).expect("failed to open Cargo.toml file");
        let cargo_toml_content_str =
            String::from_utf8(cargo_toml_content).expect("error decoding Cargo.toml utf-8");
        let toml_value = cargo_toml_content_str
            .parse::<toml::Value>()
            .expect("failed to parse Cargo.toml toml format");
        CargoTomlContents {
            path: path_ref.to_owned(),
            toml_value,
        }
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

    pub fn dependencies_table(&self) -> Option<&Table> {
        if let Some(deps) = self.toml_value.get(CARGO_TOML_DEPENDENCIES) {
            deps.as_table()
        } else {
            None
        }
    }

    pub fn dependency(&self, dep_name: &str) -> Option<&Value> {
        if let Some(deps_map) = self.dependencies_table() {
            deps_map.get(dep_name)
        } else {
            None
        }
    }

    pub fn dependencies_mut(&mut self) -> &mut Table {
        self.toml_value
            .get_mut(CARGO_TOML_DEPENDENCIES)
            .unwrap_or_else(|| panic!("no dependencies found in crate {}", self.path.display()))
            .as_table_mut()
            .expect("malformed crate Cargo.toml")
    }

    pub fn local_dependency_paths(&self, ignore_deps: &[&str]) -> Vec<String> {
        let mut result = Vec::new();
        if let Some(deps_map) = self.dependencies_table() {
            for (key, value) in deps_map {
                if ignore_deps.contains(&key.as_str()) {
                    continue;
                }

                if let Some(path) = value.get("path") {
                    result.push(path.as_str().expect("path is not a string").to_string());
                }
            }
        }
        result
    }
}
