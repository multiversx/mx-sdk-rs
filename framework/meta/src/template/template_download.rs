use std::path::PathBuf;

use toml::value::Table;

use crate::{cli_args::TemplateArgs, CargoTomlContents};

use super::{
    repo_temp_download::RepoSource,
    template_source::{template_sources, TemplateSource},
};

const ROOT_CARGO_TOML: &str = "./Cargo.toml";
const META_CARGO_TOML: &str = "./meta/Cargo.toml";
const WASM_CARGO_TOML: &str = "./wasm/Cargo.toml";

pub async fn template_download(args: &TemplateArgs) {
    let repo_temp_download = RepoSource::download_from_github(std::env::temp_dir()).await;
    let downloader = TemplateDownloader::new(
        &repo_temp_download,
        args.template.clone(),
        args.name.clone(),
    );
    downloader.template_download();
}

pub struct TemplateDownloader<'a> {
    pub repo_source: &'a RepoSource,
    pub template_source: TemplateSource<'a>,
    pub target_path: PathBuf,
}

impl<'a> TemplateDownloader<'a> {
    pub fn new(repo_source: &'a RepoSource, template_name: String, target_path: PathBuf) -> Self {
        let template_sources = template_sources(repo_source);
        let template_source = template_sources
            .into_iter()
            .find(|source| source.metadata.name == template_name)
            .unwrap_or_else(|| panic!("Unknown template {template_name}"));

        TemplateDownloader {
            repo_source,
            template_source,
            target_path,
        }
    }

    pub fn template_download(&self) {
        self.template_source.copy_template(&self.target_path);
        self.update_dependencies();
    }

    fn update_dependencies(&self) {
        self.update_dependencies_root();
        self.update_dependencies_wasm();
        self.update_dependencies_meta();
    }

    fn template_name(&self) -> String {
        self.template_source.metadata.name.clone()
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
        remove_paths_from_dependencies(deps_map, &[self.template_name().as_str()]);

        toml.save_to_file(&cargo_toml_path);
    }

    pub fn update_dependencies_wasm(&self) {
        let cargo_toml_path = self.target_path.join(WASM_CARGO_TOML);
        let mut toml = CargoTomlContents::load_from_file(&cargo_toml_path);

        let deps_map = toml.dependencies_mut();
        remove_paths_from_dependencies(deps_map, &[self.template_name().as_str()]);

        toml.save_to_file(&cargo_toml_path);
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
