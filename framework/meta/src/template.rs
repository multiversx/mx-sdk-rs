mod repo_temp_download;
mod template_list;
mod template_metadata;
mod template_metadata_load;

use crate::{cargo_toml_contents::CargoTomlContents, cli_args::TemplateArgs};
use copy_dir::*;
use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use toml::value::Table;

const REPOSITORY: &str = "https://github.com/multiversx/mx-sdk-rs/archive/refs/heads/master.zip";
const TEMPLATES_SUBDIRECTORY: &str = "mx-sdk-rs-master/contracts/examples/";
const ZIP_NAME: &str = "./master.zip";
const ROOT_CARGO_TOML: &str = "./Cargo.toml";
const META_CARGO_TOML: &str = "./meta/Cargo.toml";
const WASM_CARGO_TOML: &str = "./wasm/Cargo.toml";

pub use repo_temp_download::{RepoSource, RepoTempDownload};
pub use template_list::{list_templates, template_names};

pub struct TemplateCreator {
    pub target_path: PathBuf,
}
impl Default for TemplateCreator {
    fn default() -> Self {
        let local_path = Path::new(".").canonicalize().unwrap_or_else(|err| {
            panic!("error canonicalizing input path: {err}",);
        });
        TemplateCreator {
            target_path: local_path,
        }
    }
}

impl TemplateCreator {
    pub fn with_path(path: PathBuf) -> Self {
        TemplateCreator { target_path: path }
    }

    pub async fn download_contract_template(
        &self,
        args: &TemplateArgs,
    ) -> Result<(), reqwest::Error> {
        download_binaries().await?;
        unzip_binaries();

        let local_path = self.target_path.join(&args.name);
        copy_template_to_location(&args.name, Path::new(&local_path));
        update_dependencies(&self.target_path, &args.name);
        Ok(())
    }
}

// pub async fn list_templates() -> Result<(), reqwest::Error> {
//     download_binaries().await?;
//     unzip_binaries();

//     let contracts_path = Path::new(&env::temp_dir()).join(TEMPLATES_SUBDIRECTORY);

//     let dirs = RelevantDirectories::find_all(
//         contracts_path,
//         &["crypto-kitties".to_owned(), "order-book".to_owned()],
//     );
//     dir_pretty_print(dirs.iter_contract_crates(), "", &|_| {});
//     Ok(())
// }

pub async fn download_binaries() -> Result<(), reqwest::Error> {
    let response = reqwest::get(REPOSITORY).await?.bytes().await?;

    let tmp_dir = env::temp_dir();
    let path = tmp_dir.join(ZIP_NAME);
    let mut file = match File::create(Path::new(&path)) {
        Err(why) => panic!("couldn't create {why}"),
        Ok(file) => file,
    };
    file.write_all(&response).unwrap();
    Ok(())
}

pub fn unzip_binaries() {
    let tmp_dir = env::temp_dir();
    let path = tmp_dir.join(ZIP_NAME);
    let file = File::open(Path::new(&path)).unwrap();
    let mut zip = zip::ZipArchive::new(file).unwrap();
    zip.extract(Path::new(&tmp_dir)).unwrap();
}

pub fn copy_template_to_location(template: &str, location: &Path) {
    let contract_path = Path::new(&env::temp_dir())
        .join(TEMPLATES_SUBDIRECTORY)
        .join(template);
    let _ = copy_dir(contract_path, location);
}

pub fn update_dependencies(path: &Path, template: &str) {
    update_dependencies_root(path, template);
    update_dependencies_wasm(path, template);
    update_dependencies_meta(path, template);
}

pub fn update_dependencies_root(path: &Path, template: &str) {
    let path_buf = get_canonicalized_path(path, template, ROOT_CARGO_TOML);
    let cargo_toml_path = Path::new(&path_buf);
    let mut toml = CargoTomlContents::load_from_file(cargo_toml_path);

    let deps_map = toml.dependencies_mut();
    remove_paths_from_dependencies(deps_map, &[]);

    let dev_deps_map = toml.dev_dependencies_mut();
    remove_paths_from_dependencies(dev_deps_map, &[]);
    toml.insert_default_workspace();

    toml.save_to_file(cargo_toml_path);
}

pub fn update_dependencies_meta(path: &Path, template: &str) {
    let path_buf = get_canonicalized_path(path, template, META_CARGO_TOML);
    let cargo_toml_path = Path::new(&path_buf);
    let mut toml = CargoTomlContents::load_from_file(cargo_toml_path);

    let deps_map = toml.dependencies_mut();
    remove_paths_from_dependencies(deps_map, &[template]);

    toml.save_to_file(cargo_toml_path);
}

pub fn update_dependencies_wasm(path: &Path, template: &str) {
    let path_buf = get_canonicalized_path(path, template, WASM_CARGO_TOML);
    let cargo_toml_path = Path::new(&path_buf);
    let mut toml = CargoTomlContents::load_from_file(cargo_toml_path);

    let deps_map = toml.dependencies_mut();
    remove_paths_from_dependencies(deps_map, &[template]);

    toml.save_to_file(cargo_toml_path);
}

pub fn get_canonicalized_path(local_path: &Path, template: &str, toml: &str) -> PathBuf {
    local_path
        .join(template)
        .join(toml)
        .canonicalize()
        .unwrap_or_else(|err| {
            panic!("error canonicalizing input path: {err}",);
        })
}

pub fn remove_paths_from_dependencies(deps_map: &mut Table, ignore_deps: &[&str]) {
    for (key, value) in deps_map {
        if ignore_deps.contains(&key.as_str()) {
            continue;
        }
        value.as_table_mut().unwrap().remove("path");
    }
}
