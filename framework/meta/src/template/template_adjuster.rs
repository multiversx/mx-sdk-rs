use std::path::PathBuf;

use super::template_metadata::TemplateMetadata;
use crate::{
    cmd::standalone::upgrade::upgrade_common::{rename_files, replace_in_files},
    CargoTomlContents,
};
use convert_case::{Case, Casing};
use ruplacer::Query;
use toml::value::Table;

const TEST_DIRECTORY: &str = "./tests";
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
        self.rename_in_cargo_toml_root(&new_name);
        self.rename_in_cargo_toml_meta(&new_name);
        self.rename_in_cargo_toml_wasm(&new_name);
        self.rename_in_scenarios(&new_name);
        self.rename_in_tests(&new_name);
        self.rename_files(&new_name);
    }

    fn rename_trait_to(&self, new_template_name: &str) {
        replace_in_files(
            &self.target_path,
            "*rs",
            &[Query::substring(
                &self.metadata.contract_trait,
                new_template_name,
            )][..],
        );
    }

    fn rename_in_cargo_toml_root(&self, new_template_name: &str) {
        let old_path = self.metadata.src_file.clone();
        let mut new_path = new_template_name.to_case(Case::Snake);
        new_path.push_str(".rs");
        replace_in_files(
            &self.target_path,
            "*Cargo.toml",
            &[
                Query::substring(
                    &self.get_package_name(&self.metadata.name),
                    &self.get_package_name(new_template_name),
                ),
                Query::substring(&old_path, &new_path),
            ][..],
        );
    }
    fn rename_in_cargo_toml_meta(&self, new_template_name: &str) {
        let mut old_meta = self.metadata.name.clone();
        old_meta.push_str("-meta");
        let mut new_meta = new_template_name.to_owned();
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
                    &self.get_dependecy(&self.metadata.name),
                    &self.get_dependecy(new_template_name),
                ),
            ][..],
        );
    }
    fn rename_in_cargo_toml_wasm(&self, new_template_name: &str) {
        let mut old_wasm = self.metadata.name.clone();
        old_wasm.push_str("-wasm");
        let mut new_wasm = new_template_name.to_owned();
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
                    &self.get_dependecy(&self.metadata.name),
                    &self.get_dependecy(new_template_name),
                ),
            ][..],
        );
    }

    fn rename_in_scenarios(&self, new_template_name: &str) {
        let mut old_wasm = self.metadata.name.clone();
        old_wasm.push_str(".wasm");
        let mut new_wasm = new_template_name.to_owned();
        new_wasm.push_str(".wasm");
        replace_in_files(
            &self.target_path,
            "*.scen.json",
            &[Query::substring(
                &self.get_package_name(&old_wasm),
                &self.get_package_name(&new_wasm),
            )][..],
        );
    }
    fn rename_in_tests(&self, new_template_name: &str) {
        let new_name = new_template_name.to_case(Case::Snake);
        let old_name = self.metadata.name.to_case(Case::Snake);
        let mut new_path = "/".to_owned();
        new_path.push_str(&new_template_name);
        new_path.push('\"');
        let mut old_path = "/".to_owned();
        old_path.push_str(&self.metadata.name);
        old_path.push('\"');
        let mut new_scenarios = "scenarios/".to_owned();
        new_scenarios.push_str(&new_name);
        let mut old_scenarios = "scenarios/".to_owned();
        old_scenarios.push_str(&old_name);
        let mut new_package: String = new_name.clone();
        new_package.push_str("::");
        let mut old_package: String = old_name.clone();
        old_package.push_str("::");
        let mut old_wasm = self.metadata.name.clone();
        old_wasm.push_str(".wasm");
        let mut new_wasm = new_template_name.to_owned();
        new_wasm.push_str(".wasm");
        replace_in_files(
            &self.target_path.join(TEST_DIRECTORY),
            "*.rs",
            &[
                Query::substring(&old_wasm, &new_wasm),
                Query::substring(&old_package, &new_package),
                Query::substring(&old_path, &new_path),
                Query::substring(&old_scenarios, &new_scenarios),
            ][..],
        );
    }

    fn rename_files(&self, new_template_name: &str) {
        let new_name = new_template_name.to_case(Case::Snake);
        let mut new_src_name = new_name.clone();
        new_src_name.push_str(".rs");

        let pattern: &[(&str, &str)] = &[
            (&self.metadata.src_file, &new_src_name),
            (&self.metadata.name, &new_name),
        ];
        rename_files(&self.target_path, pattern);
    }

    fn get_package_name(&self, template: &str) -> String {
        let mut package = "name = \"".to_owned();
        package.push_str(template);
        package.push('\"');
        package
    }
    fn get_dependecy(&self, template: &str) -> String {
        let mut dependency = "dependencies.".to_owned();
        dependency.push_str(template);
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
