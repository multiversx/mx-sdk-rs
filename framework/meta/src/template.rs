use std::{env, fs::File, io::Write, path::Path};

use copy_dir::*;
use serde::{Deserialize, Serialize};

use crate::cli_args::TemplateArgs;

const REPOSITORY: &str = "https://github.com/multiversx/mx-sdk-rs/archive/refs/heads/master.zip";
const TEMPLATES_SUBDIRECTORY: &str = "mx-sdk-rs-master/contracts/examples/";
const ZIP_NAME: &str = "./master.zip";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateResponse {
    pub error: String,
    pub code: String,
    pub data: Option<Template>,
}

pub async fn download_contract_template(args: &TemplateArgs) -> Result<(), reqwest::Error> {
    download_binaries().await?;
    unzip_binaries();

    let local_path = Path::new("/").join(&args.name);
    copy_template_to_location(&args.name, &local_path);
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
