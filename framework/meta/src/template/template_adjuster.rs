use toml::value::Table;

use crate::CargoTomlContents;

use super::TemplateDownloader;

const ROOT_CARGO_TOML: &str = "./Cargo.toml";
const META_CARGO_TOML: &str = "./meta/Cargo.toml";
const WASM_CARGO_TOML: &str = "./wasm/Cargo.toml";

pub struct TemplateAdjuster;

impl TemplateAdjuster {
    pub fn update_dependencies(&self, downloader: &TemplateDownloader) {
        self.update_dependencies_root(downloader);
        self.update_dependencies_wasm(downloader);
        self.update_dependencies_meta(downloader);
    }

    fn update_dependencies_root(&self, downloader: &TemplateDownloader) {
        let cargo_toml_path = downloader.target_path.join(ROOT_CARGO_TOML);
        let mut toml = CargoTomlContents::load_from_file(&cargo_toml_path);

        let deps_map = toml.dependencies_mut();
        remove_paths_from_dependencies(deps_map, &[]);

        let dev_deps_map = toml.dev_dependencies_mut();
        remove_paths_from_dependencies(dev_deps_map, &[]);
        toml.insert_default_workspace();

        toml.save_to_file(&cargo_toml_path);
    }

    fn update_dependencies_meta(&self, downloader: &TemplateDownloader) {
        let cargo_toml_path = downloader.target_path.join(META_CARGO_TOML);
        let mut toml = CargoTomlContents::load_from_file(&cargo_toml_path);

        let deps_map = toml.dependencies_mut();
        remove_paths_from_dependencies(deps_map, &[self.template_name(downloader).as_str()]);

        toml.save_to_file(&cargo_toml_path);
    }

    fn update_dependencies_wasm(&self, downloader: &TemplateDownloader) {
        let cargo_toml_path = downloader.target_path.join(WASM_CARGO_TOML);
        let mut toml = CargoTomlContents::load_from_file(&cargo_toml_path);

        let deps_map = toml.dependencies_mut();
        remove_paths_from_dependencies(deps_map, &[self.template_name(downloader).as_str()]);

        toml.save_to_file(&cargo_toml_path);
    }

    fn template_name(&self, downloader: &TemplateDownloader) -> String {
        downloader.template_source.metadata.name.clone()
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
