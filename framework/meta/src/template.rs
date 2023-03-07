use crate::{cargo_toml_contents::CargoTomlContents, cli_args::TemplateArgs};
use copy_dir::*;
use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

const REPOSITORY: &str = "https://github.com/multiversx/mx-sdk-rs/archive/refs/heads/master.zip";
const TEMPLATES_SUBDIRECTORY: &str = "mx-sdk-rs-master/contracts/examples/";
const ZIP_NAME: &str = "./master.zip";
const ROOT_CARGO_TOML: &str = "./Cargo.toml";
const META_CARGO_TOML: &str = "./meta/Cargo.toml";
const WASM_CARGO_TOML: &str = "./wasm/Cargo.toml";

pub async fn download_contract_template(args: &TemplateArgs) -> Result<(), reqwest::Error> {
    download_binaries().await?;
    unzip_binaries();

    let local_path = Path::new(".")
        .canonicalize()
        .unwrap_or_else(|err| {
            panic!("error canonicalizing input path: {err}",);
        })
        .join(&args.name);
    copy_template_to_location(&args.name, Path::new(&local_path));
    update_dependencies(&args.name);
    Ok(())
}

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

pub fn update_dependencies(template: &str) {
    update_dependencies_root(template);
    update_dependencies_wasm(template);
    update_dependencies_meta(template);
}

pub fn update_dependencies_root(template: &str) {
    let path_buf = get_canonicalized_path(template, ROOT_CARGO_TOML);
    let root_cargo_toml_path = Path::new(&path_buf);
    let mut toml = CargoTomlContents::load_from_file(root_cargo_toml_path);

    toml.dev_dependencies_mut().clear();

    remove_paths_from_dependencies(&mut toml, root_cargo_toml_path, &[]);
}

pub fn update_dependencies_meta(template: &str) {
    let path_buf = get_canonicalized_path(template, META_CARGO_TOML);
    let root_cargo_toml_path = Path::new(&path_buf);
    let mut toml = CargoTomlContents::load_from_file(root_cargo_toml_path);

    remove_paths_from_dependencies(&mut toml, root_cargo_toml_path, &[template]);
}

pub fn update_dependencies_wasm(template: &str) {
    let path_buf = get_canonicalized_path(template, WASM_CARGO_TOML);
    let root_cargo_toml_path = Path::new(&path_buf);
    let mut toml = CargoTomlContents::load_from_file(root_cargo_toml_path);

    remove_paths_from_dependencies(&mut toml, root_cargo_toml_path, &[template]);
}

pub fn get_canonicalized_path(template: &str, toml: &str) -> PathBuf {
    Path::new(template)
        .join(toml)
        .canonicalize()
        .unwrap_or_else(|err| {
            panic!("error canonicalizing input path: {err}",);
        })
}

pub fn remove_paths_from_dependencies(
    cargo_toml_contents: &mut CargoTomlContents,
    path: &Path,
    ignore_deps: &[&str],
) {
    let deps_map = cargo_toml_contents.dependencies_mut();
    for (key, value) in deps_map {
        if ignore_deps.contains(&key.as_str()) {
            continue;
        }
        value.as_table_mut().unwrap().remove("path");
    }

    cargo_toml_contents.save_to_file(path);
}
