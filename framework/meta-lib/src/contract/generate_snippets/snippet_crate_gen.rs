use colored::Colorize;
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
};

use crate::version_history;

static SNIPPETS_SOURCE_FILE_NAME: &str = "interactor_main.rs";
static LIB_SOURCE_FILE_NAME: &str = "interact.rs";
static SC_CONFIG_PATH: &str = "../sc-config.toml";
static CONFIG_TOML_PATH: &str = "config.toml";
static CONFIG_SOURCE_FILE_NAME: &str = "config.rs";
static FULL_PROXY_ENTRY: &str = r#"[[proxy]]
path = "interactor/src/proxy.rs"
 "#;
static PROXY_PATH: &str = "interactor/src/proxy.rs";

pub(crate) fn create_snippets_folder(snippets_folder_path: &str) {
    // returns error if folder already exists, so we ignore the result
    let _ = fs::create_dir(snippets_folder_path);
}

pub(crate) fn create_snippets_gitignore(snippets_folder_path: &str, overwrite: bool) {
    let gitignore_path = format!("{snippets_folder_path}/.gitignore");
    let mut file = if overwrite {
        File::create(&gitignore_path).unwrap()
    } else {
        match File::options()
            .create_new(true)
            .write(true)
            .open(&gitignore_path)
        {
            Ok(f) => f,
            Err(_) => return,
        }
    };

    writeln!(
        &mut file,
        "# Pem files are used for interactions, but shouldn't be committed
*.pem"
    )
    .unwrap();
}

pub(crate) fn create_snippets_cargo_toml(
    snippets_folder_path: &str,
    contract_crate_name: &str,
    overwrite: bool,
) {
    let cargo_toml_path = format!("{snippets_folder_path}/Cargo.toml");
    let mut file = if overwrite {
        File::create(&cargo_toml_path).unwrap()
    } else {
        match File::options()
            .create_new(true)
            .write(true)
            .open(&cargo_toml_path)
        {
            Ok(f) => f,
            Err(_) => return,
        }
    };

    let last_release_version = &version_history::LAST_VERSION;

    writeln!(
        &mut file,
        r#"[package]
name = "rust-interact"
version = "0.0.0"
authors = ["you"]
edition = "2021"
publish = false

[[bin]]
name = "rust-interact"
path = "src/{SNIPPETS_SOURCE_FILE_NAME}"

[lib]
path = "src/{LIB_SOURCE_FILE_NAME}"

[dependencies.{contract_crate_name}]
path = ".."

[dependencies.multiversx-sc-snippets]
version = "{last_release_version}"

[dependencies.multiversx-sc]
version = "{last_release_version}"

[dependencies]
clap = {{ version = "4.4.7", features = ["derive"] }}
serde = {{ version = "1.0", features = ["derive"] }}
toml = "0.8.6"

[features]
chain-simulator-tests = []
"#
    )
    .unwrap();
}

pub(crate) fn create_src_folder(snippets_folder_path: &str) {
    // returns error if folder already exists, so we ignore the result
    let src_folder_path = format!("{snippets_folder_path}/src");
    let _ = fs::create_dir(src_folder_path);
}

#[must_use]
pub(crate) fn create_and_get_lib_file(snippets_folder_path: &str, overwrite: bool) -> File {
    let lib_path = format!("{snippets_folder_path}/src/{LIB_SOURCE_FILE_NAME}");
    if overwrite {
        File::create(&lib_path).unwrap()
    } else {
        match File::options().create_new(true).write(true).open(&lib_path) {
            Ok(f) => f,
            Err(_) => {
                println!(
                    "{}",
                    format!("{lib_path} file already exists, --overwrite option was not provided",)
                        .yellow()
                );
                File::options().write(true).open(&lib_path).unwrap()
            },
        }
    }
}

pub(crate) fn create_main_file(snippets_folder_path: &str, contract_crate_name: &str) {
    let lib_path = format!("{snippets_folder_path}/src/{SNIPPETS_SOURCE_FILE_NAME}");

    let mut file = File::create(&lib_path).unwrap();

    writeln!(
        &mut file,
        r#"
use multiversx_sc_snippets::imports::*;
use rust_interact::{contract_crate_name}_cli;

#[tokio::main]
async fn main() {{
    {contract_crate_name}_cli().await;
}}  
"#
    )
    .unwrap();
}

pub(crate) fn create_sc_config_file(overwrite: bool) {
    // check if the file should be overwritten or if it already exists
    let mut file = if overwrite || !file_exists(SC_CONFIG_PATH) {
        File::create(SC_CONFIG_PATH).unwrap()
    } else {
        // file already exists
        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(SC_CONFIG_PATH)
            .unwrap();

        if file_contains_proxy_path(SC_CONFIG_PATH).unwrap_or(false) {
            return;
        }

        file
    };

    // write full proxy toml entry to the file
    writeln!(&mut file, "\n{FULL_PROXY_ENTRY}").unwrap();
}

pub(crate) fn create_config_toml_file(snippets_folder_path: &str) {
    let config_path = format!("{snippets_folder_path}/{CONFIG_TOML_PATH}");
    let mut file = File::create(config_path).unwrap();

    writeln!(
        &mut file,
        r#"
# chain_type = 'simulator'
# gateway_uri = 'http://localhost:8085'

chain_type = 'real'
gateway_uri = 'https://devnet-gateway.multiversx.com'
"#
    )
    .unwrap();
}

pub(crate) fn create_config_rust_file(snippets_folder_path: &str) -> File {
    let lib_path = format!("{snippets_folder_path}/src/{CONFIG_SOURCE_FILE_NAME}");

    File::create(lib_path).unwrap()
}

fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn file_contains_proxy_path(file_path: &str) -> std::io::Result<bool> {
    let file_content = fs::read_to_string(file_path)?;
    let proxy_entry = format!("path = \"{}\"", PROXY_PATH);

    Ok(file_content.contains(&proxy_entry))
}
