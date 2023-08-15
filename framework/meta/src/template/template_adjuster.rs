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
        self.rename_in_tests(&new_name);
        self.rename_solution_files(&new_name);
    }

    fn rename_trait_to(&self, new_template_name: &str) {
        let new_name = new_template_name.to_case(Case::Snake);
        let old_name = self.metadata.name.to_case(Case::Snake);
        let new_package = format!("{new_name}::");
        let old_package = format!("{old_name}::");

        replace_in_files(
            &self.target_path,
            "*rs",
            &[
                Query::substring(&self.metadata.contract_trait, new_template_name),
                Query::substring(&old_package, &new_package),
            ][..],
        );
    }

    fn rename_in_cargo_toml_root(&self, new_template_name: &str) {
        let old_path = self.metadata.src_file.clone();
        let new_path = rs_file_name(&new_template_name.to_case(Case::Snake));

        replace_in_files(
            &self.target_path,
            "*Cargo.toml",
            &[
                Query::substring(
                    &package_name_expr(&self.metadata.name),
                    &package_name_expr(new_template_name),
                ),
                Query::substring(&old_path, &new_path),
            ][..],
        );
    }

    fn rename_in_cargo_toml_meta(&self, new_template_name: &str) {
        let old_meta = format!("{}-meta", self.metadata.name.clone());
        let new_meta = format!("{}-meta", new_template_name.to_owned());

        replace_in_files(
            &self.target_path,
            "*Cargo.toml",
            &[
                Query::substring(&package_name_expr(&old_meta), &package_name_expr(&new_meta)),
                Query::substring(
                    &dependecy_decl_expr(&self.metadata.name),
                    &dependecy_decl_expr(new_template_name),
                ),
            ][..],
        );
    }

    fn rename_in_cargo_toml_wasm(&self, new_template_name: &str) {
        let old_wasm = format!("{}-wasm", self.metadata.name.clone());
        let new_wasm = format!("{}-wasm", new_template_name.to_owned());

        replace_in_files(
            &self.target_path,
            "*Cargo.toml",
            &[
                Query::substring(&package_name_expr(&old_wasm), &package_name_expr(&new_wasm)),
                Query::substring(
                    &dependecy_decl_expr(&self.metadata.name),
                    &dependecy_decl_expr(new_template_name),
                ),
            ][..],
        );
    }

    fn rename_in_tests(&self, new_template_name: &str) {
        let new_name = new_template_name.to_case(Case::Snake);
        let old_name = self.metadata.name.to_case(Case::Snake);

        let mut queries = Vec::<Query>::new();
        for (old, new) in self.metadata.rename_pairs.iter() {
            queries.push(Query::substring(old, new))
        }

        let new_path = as_path(new_template_name);
        let old_path = as_path(&self.metadata.name);
        queries.push(Query::substring(&old_path, &new_path));

        let new_scenarios = scenario_path(&new_name);
        let old_scenarios = scenario_path(&old_name);
        queries.push(Query::substring(&old_scenarios, &new_scenarios));

        let old_wasm = wasm_file_name(&self.metadata.name);
        let new_wasm = wasm_file_name(new_template_name);

        replace_in_files(
            &self.target_path,
            "*.scen.json",
            &[Query::substring(&old_wasm, &new_wasm)][..],
        );

        queries.push(Query::substring(&old_wasm, &new_wasm));

        replace_in_files(&self.target_path.join(TEST_DIRECTORY), "*.rs", &queries);
    }

    fn rename_solution_files(&self, new_template_name: &str) {
        let new_name = new_template_name.to_case(Case::Snake);
        let new_src_name = rs_file_name(&new_name);

        let pattern: &[(&str, &str)] = &[
            (&self.metadata.src_file, &new_src_name),
            (&self.metadata.name, &new_name),
        ];
        rename_files(&self.target_path, pattern);
    }
}

fn wasm_file_name(name: &str) -> String {
    format!("{name}.wasm",)
}

fn rs_file_name(name: &str) -> String {
    format!("{name}.rs",)
}

fn scenario_path(path: &str) -> String {
    format!("scenarios/{path}",)
}

fn as_path(name: &str) -> String {
    format!("/{name}\"")
}
fn package_name_expr(template: &str) -> String {
    format!("name = \"{template}\"")
}
fn dependecy_decl_expr(template: &str) -> String {
    format!("dependencies.{template}")
}

pub fn remove_paths_from_dependencies(deps_map: &mut Table, ignore_deps: &[&str]) {
    for (key, value) in deps_map {
        if ignore_deps.contains(&key.as_str()) {
            continue;
        }
        if let Some(dep) = value.as_table_mut() {
            dep.remove("path");
        }
    }
}
