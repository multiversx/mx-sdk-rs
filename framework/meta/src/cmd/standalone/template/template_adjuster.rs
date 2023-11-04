use super::{template_metadata::TemplateMetadata, ContractCreatorTarget};
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
    pub metadata: TemplateMetadata,
    pub target: ContractCreatorTarget,
    pub keep_paths: bool,
}
impl TemplateAdjuster {
    pub fn update_dependencies(&self) {
        self.update_dependencies_root();
        self.update_dependencies_wasm();
        self.update_dependencies_meta();
    }

    fn update_dependencies_root(&self) {
        let cargo_toml_path = self.target.contract_dir().join(ROOT_CARGO_TOML);
        let mut toml = CargoTomlContents::load_from_file(&cargo_toml_path);

        if !self.keep_paths {
            remove_paths_from_deps(&mut toml, &[]);
        }

        toml.insert_default_workspace();

        toml.save_to_file(&cargo_toml_path);
    }

    fn update_dependencies_meta(&self) {
        let cargo_toml_path = self.target.contract_dir().join(META_CARGO_TOML);
        let mut toml = CargoTomlContents::load_from_file(&cargo_toml_path);

        if !self.keep_paths {
            remove_paths_from_deps(&mut toml, &[&self.metadata.name]);
        }

        toml.save_to_file(&cargo_toml_path);
    }

    fn update_dependencies_wasm(&self) {
        let cargo_toml_path = self.target.contract_dir().join(WASM_CARGO_TOML);
        let mut toml = CargoTomlContents::load_from_file(&cargo_toml_path);

        if !self.keep_paths {
            remove_paths_from_deps(&mut toml, &[&self.metadata.name]);
        }

        toml.save_to_file(&cargo_toml_path);
    }

    pub fn rename_template_to(&self) {
        self.rename_trait_to();
        self.rename_in_cargo_toml_root();
        self.rename_in_cargo_toml_meta();
        self.rename_in_cargo_toml_wasm();
        self.rename_in_tests();
        self.rename_solution_files();
    }

    fn rename_trait_to(&self) {
        let new_trait = self.target.new_name.to_case(Case::UpperCamel);
        let old_trait = &self.metadata.contract_trait;
        let new_name = self.target.new_name.to_case(Case::Snake);
        let old_name = self.metadata.name.to_case(Case::Snake);
        let new_package = format!("{new_name}::");
        let old_package = format!("{old_name}::");

        replace_in_files(
            &self.target.contract_dir(),
            "*rs",
            &[
                Query::substring(old_trait, &new_trait),
                Query::substring(&old_package, &new_package),
            ][..],
        );
    }

    fn rename_in_cargo_toml_root(&self) {
        let old_path = self.metadata.src_file.clone();
        let new_path = rs_file_name(&self.target.new_name.to_case(Case::Snake));

        replace_in_files(
            &self.target.contract_dir(),
            "*Cargo.toml",
            &[
                Query::substring(
                    &package_name_expr(&self.metadata.name),
                    &package_name_expr(&self.target.new_name),
                ),
                Query::substring(&old_path, &new_path),
            ][..],
        );
    }

    fn rename_in_cargo_toml_meta(&self) {
        let old_meta = format!("{}-meta", self.metadata.name.clone());
        let new_meta = format!("{}-meta", &self.target.new_name);

        replace_in_files(
            &self.target.contract_dir(),
            "*Cargo.toml",
            &[
                Query::substring(&package_name_expr(&old_meta), &package_name_expr(&new_meta)),
                Query::substring(
                    &dependecy_decl_expr(&self.metadata.name),
                    &dependecy_decl_expr(&self.target.new_name),
                ),
            ][..],
        );
    }

    fn rename_in_cargo_toml_wasm(&self) {
        let old_wasm = format!("{}-wasm", self.metadata.name.clone());
        let new_wasm = format!("{}-wasm", &self.target.new_name);

        replace_in_files(
            &self.target.contract_dir(),
            "*Cargo.toml",
            &[
                Query::substring(&package_name_expr(&old_wasm), &package_name_expr(&new_wasm)),
                Query::substring(
                    &dependecy_decl_expr(&self.metadata.name),
                    &dependecy_decl_expr(&self.target.new_name),
                ),
            ][..],
        );
    }

    fn rename_in_tests(&self) {
        let new_name = self.target.new_name.to_case(Case::Snake);
        let old_name = self.metadata.name.to_case(Case::Snake);

        let mut queries = Vec::<Query>::new();
        for (old, new) in self.metadata.rename_pairs.iter() {
            queries.push(Query::substring(old, new))
        }

        let new_path = as_path(&self.target.new_name);
        let old_path = as_path(&self.metadata.name);
        queries.push(Query::substring(&old_path, &new_path));

        let new_scenarios = scenario_path(&new_name);
        let old_scenarios = scenario_path(&old_name);
        queries.push(Query::substring(&old_scenarios, &new_scenarios));

        let old_wasm = wasm_file_name(&self.metadata.name);
        let new_wasm = wasm_file_name(&self.target.new_name);

        replace_in_files(
            &self.target.contract_dir(),
            "*.scen.json",
            &[Query::substring(&old_wasm, &new_wasm)][..],
        );

        queries.push(Query::substring(&old_wasm, &new_wasm));

        replace_in_files(
            &self.target.contract_dir().join(TEST_DIRECTORY),
            "*.rs",
            &queries,
        );
    }

    fn rename_solution_files(&self) {
        let new_name = self.target.new_name.to_case(Case::Snake);
        let new_src_name = rs_file_name(&new_name);

        let pattern: &[(&str, &str)] = &[
            (&self.metadata.src_file, &new_src_name),
            (&self.metadata.name, &new_name),
        ];
        rename_files(&self.target.contract_dir(), pattern);
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

pub fn remove_paths_from_deps_map(deps_map: &mut Table, ignore_deps: &[&str]) {
    for (key, value) in deps_map {
        if ignore_deps.contains(&key.as_str()) {
            continue;
        }
        if let Some(dep) = value.as_table_mut() {
            dep.remove("path");
        }
    }
}

pub fn remove_paths_from_deps(toml: &mut CargoTomlContents, ignore_deps: &[&str]) {
    if toml.has_dependencies() {
        remove_paths_from_deps_map(toml.dependencies_mut(), ignore_deps);
    }
    if toml.has_dev_dependencies() {
        remove_paths_from_deps_map(toml.dev_dependencies_mut(), ignore_deps);
    }
}
