use std::path::PathBuf;

use super::template_metadata::TemplateMetadata;
use crate::{cmd::standalone::upgrade::upgrade_common::replace_in_files, CargoTomlContents};
use convert_case::{Case, Casing};
use ruplacer::Query;
use toml::value::Table;

const ROOT_CARGO_TOML: &str = "./Cargo.toml";
const META_CARGO_TOML: &str = "./meta/Cargo.toml";
const WASM_CARGO_TOML: &str = "./wasm/Cargo.toml";

pub struct TemplateAdjuster {
    pub target_path: PathBuf,
    pub metadata: TemplateMetadata,
}
impl TemplateAdjuster {
    pub fn new(target_path: PathBuf, metadata: TemplateMetadata) -> Self {
        Self {
            target_path,
            metadata,
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
        remove_paths_from_dependencies(deps_map, &[&self.metadata.name]);

        toml.save_to_file(&cargo_toml_path);
    }

    fn update_dependencies_wasm(&self) {
        let cargo_toml_path = self.target_path.join(WASM_CARGO_TOML);
        let mut toml = CargoTomlContents::load_from_file(&cargo_toml_path);

        let deps_map = toml.dependencies_mut();
        remove_paths_from_dependencies(deps_map, &[&self.metadata.name]);

        toml.save_to_file(&cargo_toml_path);
    }

    pub fn rename_template_to(&self, new_name: String) {
        self.rename_trait_to(&new_name.to_case(Case::UpperCamel));
        self.rename_cargo_toml_root(&new_name);
        self.rename_cargo_toml_meta(&new_name);
        self.rename_cargo_toml_wasm(&new_name);
    }

    fn rename_trait_to(&self, new_template_name: &String) {
        replace_in_files(
            &self.target_path,
            "*rs",
            &[Query::substring(
                &self.metadata.contract_trait,
                new_template_name,
            )][..],
        );
    }

    fn rename_cargo_toml_root(&self, new_template_name: &String) {
        replace_in_files(
            &self.target_path,
            "*Cargo.toml",
            &[Query::substring(
                &self.get_package_name(&self.metadata.name),
                &self.get_package_name(new_template_name),
            )][..],
        );
    }
    fn rename_cargo_toml_meta(&self, new_template_name: &String) {
        let mut old_meta = self.metadata.name.clone();
        old_meta.push_str("-meta");
        let mut new_meta = new_template_name.clone();
        new_meta.push_str("-meta");
        replace_in_files(
            &self.target_path,
            "*Cargo.toml",
            &[
                Query::substring(
                    &self.get_package_name(&old_meta),
                    &self.get_package_name(&new_meta),
                ),
                Query::substring(
                    &self.get_dependecy(&self.metadata.name.clone()),
                    &self.get_dependecy(&new_template_name),
                ),
            ][..],
        );
    }
    fn rename_cargo_toml_wasm(&self, new_template_name: &String) {
        let mut old_wasm = self.metadata.name.clone();
        old_wasm.push_str("-wasm");
        let mut new_wasm = new_template_name.clone();
        new_wasm.push_str("-wasm");
        replace_in_files(
            &self.target_path,
            "*Cargo.toml",
            &[
                Query::substring(
                    &self.get_package_name(&old_wasm),
                    &self.get_package_name(&new_wasm),
                ),
                Query::substring(
                    &self.get_dependecy(&self.metadata.name.clone()),
                    &self.get_dependecy(&new_template_name),
                ),
            ][..],
        );
    }

    fn get_package_name(&self, template: &String) -> String {
        let mut package = "name =\"".to_owned();
        package.push_str(template);
        package.push_str("\"");
        package
    }
    fn get_dependecy(&self, template: &String) -> String {
        let mut dependency = "dependencies.".to_owned();
        dependency.push_str(&template);
        dependency
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
