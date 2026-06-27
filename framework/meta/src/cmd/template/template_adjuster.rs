use std::path::{Path, PathBuf};

use super::{ContractCreatorTarget, template_metadata::TemplateMetadata};
use crate::cmd::upgrade::upgrade_common::{rename_files, replace_in_files};
use convert_case::{Case, Casing};
use multiversx_sc_meta_lib::cargo_toml::{CargoTomlContents, WorkspaceDependencies};
use ruplacer::Query;
use toml::value::Table;

const SNIPPETS_DIRECTORY_NAME: &str = "snippets";
const CARGO_TOML: &str = "Cargo.toml";
const DEFAULT_AUTHOR: &str = "you";

pub struct TemplateAdjuster {
    pub metadata: TemplateMetadata,
    pub target: ContractCreatorTarget,
    pub keep_paths: bool,
    pub new_author: Option<String>,
    pub workspace_dependencies: WorkspaceDependencies,
}
impl TemplateAdjuster {
    pub fn resolve_workspace_dependencies(&self) {
        for cargo_toml_path in self.cargo_toml_paths() {
            if !cargo_toml_path.exists() {
                continue;
            }

            let mut toml = CargoTomlContents::load_from_file(&cargo_toml_path);
            toml.resolve_workspace_dependencies(&self.workspace_dependencies);
            toml.save_to_file(&cargo_toml_path);
        }
    }

    pub fn update_cargo_toml_files(&self) {
        let author_as_str = self
            .new_author
            .clone()
            .unwrap_or_else(|| DEFAULT_AUTHOR.to_string());
        self.update_cargo_toml_root(author_as_str.clone());
        self.update_cargo_toml_meta();
        self.update_cargo_toml_wasm();
        self.update_cargo_toml_interact(author_as_str);
    }

    fn cargo_toml_paths(&self) -> Vec<PathBuf> {
        let mut paths = vec![
            self.target.contract_dir().join(CARGO_TOML),
            self.target.contract_dir().join("meta").join(CARGO_TOML),
            self.target.contract_dir().join("wasm").join(CARGO_TOML),
        ];
        if self.metadata.has_interactor {
            paths.push(
                self.target
                    .contract_dir()
                    .join("interactor")
                    .join(CARGO_TOML),
            );
        }
        paths
    }

    fn update_cargo_toml_root(&self, author: String) {
        let cargo_toml_path = self.target.contract_dir().join(CARGO_TOML);
        let mut toml = CargoTomlContents::load_from_file(&cargo_toml_path);

        if !self.keep_paths {
            remove_paths_from_deps(&mut toml, &[]);
        }

        if self.metadata.has_interactor {
            toml.add_workspace(&[".", "meta", "interactor"]);
        } else {
            toml.add_workspace(&[".", "meta"]);
        }
        toml.change_author(author);
        toml.save_to_file(&cargo_toml_path);
    }

    fn update_cargo_toml_meta(&self) {
        let cargo_toml_path = self.target.contract_dir().join("meta").join(CARGO_TOML);
        let mut toml = CargoTomlContents::load_from_file(&cargo_toml_path);

        if !self.keep_paths {
            remove_paths_from_deps(&mut toml, &[&self.metadata.name]);
        }

        toml.save_to_file(&cargo_toml_path);
    }

    fn update_cargo_toml_wasm(&self) {
        let cargo_toml_path = self.target.contract_dir().join("wasm").join(CARGO_TOML);
        if !cargo_toml_path.exists() {
            return;
        }
        let mut toml = CargoTomlContents::load_from_file(&cargo_toml_path);

        if !self.keep_paths {
            remove_paths_from_deps(&mut toml, &[&self.metadata.name]);
        }

        toml.save_to_file(&cargo_toml_path);
    }

    fn update_cargo_toml_interact(&self, author: String) {
        if !self.metadata.has_interactor {
            return;
        }

        let cargo_toml_path = self
            .target
            .contract_dir()
            .join("interactor")
            .join(CARGO_TOML);
        let mut toml = CargoTomlContents::load_from_file(&cargo_toml_path);

        if !self.keep_paths {
            remove_paths_from_deps(&mut toml, &[&self.metadata.name]);
        }

        toml.change_author(author);
        toml.save_to_file(&cargo_toml_path);
    }

    pub fn rename_template_to(&self) {
        self.rename_in_files();
        self.rename_solution_files();
    }

    fn rename_in_files(&self) {
        let new_snake = self.target.new_name.to_case(Case::Snake);
        let old_snake = self.metadata.name.to_case(Case::Snake);
        let new_trait = self.target.new_name.to_case(Case::UpperCamel);
        let old_trait = &self.metadata.contract_trait;
        let new_src_file = rs_file_name(&new_snake);
        let old_wasm = wasm_file_name(&self.metadata.name);
        let new_wasm = wasm_file_name(&self.target.new_name);
        let old_mxsc = mxsc_file_name(&self.metadata.name);
        let new_mxsc = mxsc_file_name(&self.target.new_name);

        // All .rs replacements in a single pass from the contract root.
        // Queries that only apply to specific subdirectories (tests, interactor) are
        // harmless no-ops in other .rs files because their patterns are specific enough.
        // rename_pairs must come first: later queries (e.g. as_path) can corrupt their patterns.
        let mut rs_queries = Vec::<Query>::new();
        for (old, new) in &self.metadata.rename_pairs {
            rs_queries.push(Query::simple(old, new));
        }
        rs_queries.extend([
            Query::simple(old_trait, &new_trait),
            Query::simple(&format!("{old_snake}::"), &format!("{new_snake}::")),
            Query::simple(&format!("{old_snake}_proxy"), &format!("{new_snake}_proxy")),
            Query::simple(
                &as_path(&self.metadata.name),
                &as_path(&self.target.new_name),
            ),
            Query::simple(&scenario_path(&old_snake), &scenario_path(&new_snake)),
            Query::simple(&old_wasm, &new_wasm),
            Query::simple(&old_mxsc, &new_mxsc),
        ]);
        replace_in_files(&self.target.contract_dir(), "*rs", &rs_queries);

        replace_in_files(
            &self.target.contract_dir(),
            "*sc-config.toml",
            &[Query::simple(
                &format!("{old_snake}_proxy"),
                &format!("{new_snake}_proxy"),
            )],
        );

        replace_in_files(
            &self.target.contract_dir(),
            "*Cargo.toml",
            &[
                Query::simple(
                    &package_name_expr(&self.metadata.name),
                    &package_name_expr(&self.target.new_name),
                ),
                Query::simple(&self.metadata.src_file, &new_src_file),
                Query::simple(
                    &package_name_expr(&format!("{}-meta", self.metadata.name)),
                    &package_name_expr(&format!("{}-meta", self.target.new_name)),
                ),
                Query::simple(
                    &dependency_decl_expr(&self.metadata.name),
                    &dependency_decl_expr(&self.target.new_name),
                ),
                Query::simple(
                    &package_name_expr(&format!("{}-wasm", self.metadata.name)),
                    &package_name_expr(&format!("{}-wasm", self.target.new_name)),
                ),
            ],
        );

        let output_path_queries = &[
            Query::simple(&old_wasm, &new_wasm),
            Query::simple(&old_mxsc, &new_mxsc),
        ];
        replace_in_files(
            &self.target.contract_dir(),
            "*.scen.json",
            output_path_queries,
        );
        replace_in_files(
            &self.target.contract_dir(),
            "*.steps.json",
            output_path_queries,
        );

        replace_in_files(
            &self.target.contract_dir().join(SNIPPETS_DIRECTORY_NAME),
            "*.sh",
            output_path_queries,
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

fn mxsc_file_name(name: &str) -> String {
    format!("{name}.mxsc.json",)
}

fn rs_file_name(name: &str) -> String {
    format!("{name}.rs",)
}

fn scenario_path(path: &str) -> String {
    Path::new("scenarios")
        .join(path)
        .to_string_lossy()
        .to_string()
}

fn as_path(name: &str) -> String {
    format!("/{name}\"")
}
fn package_name_expr(template: &str) -> String {
    format!("name = \"{template}\"")
}
fn dependency_decl_expr(template: &str) -> String {
    format!("[dependencies.{template}]")
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
