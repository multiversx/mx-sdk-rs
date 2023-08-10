use std::path::PathBuf;

use crate::{cmd::standalone::upgrade::upgrade_common::replace_in_files, CargoTomlContents};
use ruplacer::Query;
use toml::value::Table;

const TEMPLATE_TOML: &str = "./template.toml";
const ROOT_CARGO_TOML: &str = "./Cargo.toml";
const META_CARGO_TOML: &str = "./meta/Cargo.toml";
const WASM_CARGO_TOML: &str = "./wasm/Cargo.toml";

pub struct TemplateAdjuster {
    pub target_path: PathBuf,
    pub template_name: String,
    pub contract_trait: String,
    pub src_file: String,
    pub package_name: String,
    pub rename_pairs: Vec<String>,
}

impl TemplateAdjuster {
    pub fn new(target_path: PathBuf, template_name: String) -> Self {
        let cargo_toml_path = target_path.join(TEMPLATE_TOML);
        let toml = CargoTomlContents::load_from_file(&cargo_toml_path);
        Self {
            target_path,
            template_name,
            contract_trait: toml
                .toml_value
                .get("contract_trait")
                .expect("missing contract_trait in template.toml")
                .as_str()
                .expect("contract_trait not a string value")
                .to_string(),
            src_file: toml
                .toml_value
                .get("contract_trait")
                .expect("missing src_file in template.toml")
                .as_str()
                .expect("src_file not a string value")
                .to_string(),
            package_name: toml
                .toml_value
                .get("contract_trait")
                .expect("missing package_name in template.toml")
                .as_str()
                .expect("package_name not a string value")
                .to_string(),
            rename_pairs: toml
                .toml_value
                .get("rename_pairs")
                .expect("missing contract_trait in template.toml")
                .as_array()
                .expect("package_name not an array value")
                .iter()
                .map(|value| value.to_string())
                .collect(),
        }
    }

    pub fn update_dependencies(&self) {
        self.update_dependencies_root();
        self.update_dependencies_wasm();
        self.update_dependencies_meta();
    }

    fn update_dependencies_root(&self) {
        let cargo_toml_path = self.target_path.join(ROOT_CARGO_TOML);
        let mut toml = CargoTomlContents::load_from_file(&cargo_toml_path);

        let deps_map = toml.dependencies_mut();
        remove_paths_from_dependencies(deps_map, &[]);

        let dev_deps_map = toml.dev_dependencies_mut();
        remove_paths_from_dependencies(dev_deps_map, &[]);
        toml.insert_default_workspace();

        toml.save_to_file(&cargo_toml_path);
    }

    fn update_dependencies_meta(&self) {
        let cargo_toml_path = self.target_path.join(META_CARGO_TOML);
        let mut toml = CargoTomlContents::load_from_file(&cargo_toml_path);

        let deps_map = toml.dependencies_mut();
        remove_paths_from_dependencies(deps_map, &[&self.template_name]);

        toml.save_to_file(&cargo_toml_path);
    }

    fn update_dependencies_wasm(&self) {
        let cargo_toml_path = self.target_path.join(WASM_CARGO_TOML);
        let mut toml = CargoTomlContents::load_from_file(&cargo_toml_path);

        let deps_map = toml.dependencies_mut();
        remove_paths_from_dependencies(deps_map, &[&self.template_name]);

        toml.save_to_file(&cargo_toml_path);
    }

    pub fn rename_trait_to(&self, new_template_name: String) {
        let cargo_toml_path = self.target_path.join(TEMPLATE_TOML);
        let toml = CargoTomlContents::load_from_file(&cargo_toml_path);

        let contract_trait = toml
            .toml_value
            .get("contract_trait")
            .expect("missing contract_trait in template.toml")
            .as_str()
            .expect("contract_trait not a string value")
            .to_string();

        replace_in_files(
            &self.target_path,
            "*rs",
            &[Query::substring(&contract_trait, &new_template_name)][..],
        );
    }
}
pub fn remove_paths_from_dependencies(deps_map: &mut Table, ignore_deps: &[&str]) {
    for (key, value) in deps_map {
        if ignore_deps.contains(&key.as_str()) {
            continue;
        }
        value.as_table_mut().unwrap().remove("path");
    }
}
