use colored::Colorize;
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use crate::version_history;

static SNIPPETS_SOURCE_FILE_NAME: &str = "interactor_main.rs";
static LIB_SOURCE_FILE_NAME: &str = "interact.rs";
static SC_CONFIG_FILE_NAME: &str = "sc-config.toml";
static CONFIG_TOML_PATH: &str = "config.toml";
static CONFIG_SOURCE_FILE_NAME: &str = "config.rs";
static INTERACTOR_CS_TEST_FILE_NAME: &str = "interact_cs_tests.rs";
static INTERACTOR_TEST_FILE_NAME: &str = "interact_tests.rs";

pub(crate) fn create_snippets_folder(snippets_folder_path: &PathBuf) {
    let _ = fs::create_dir(snippets_folder_path);
}

pub(crate) fn create_snippets_gitignore(snippets_folder_path: &Path, overwrite: bool) {
    let gitignore_path = snippets_folder_path.join(".gitignore");
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
    snippets_folder_path: &Path,
    contract_crate_name: &str,
    overwrite: bool,
) {
    let contract_deps = contract_crate_name.replace("_", "-");
    let cargo_toml_path = snippets_folder_path.join("Cargo.toml");
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
edition = "2024"
publish = false

[[bin]]
name = "rust-interact"
path = "src/{SNIPPETS_SOURCE_FILE_NAME}"

[lib]
path = "src/{LIB_SOURCE_FILE_NAME}"

[dependencies.{contract_deps}]
path = ".."

[dependencies.multiversx-sc-snippets]
version = "{last_release_version}"

[dependencies.multiversx-sc]
version = "{last_release_version}"

[dependencies]
clap = {{ version = "4.4.7", features = ["derive"] }}
serde = {{ version = "1.0", features = ["derive"] }}
toml = "0.9"

[features]
chain-simulator-tests = []
"#
    )
    .unwrap();
}

pub(crate) fn create_src_folder(snippets_folder_path: &Path) {
    // returns error if folder already exists, so we ignore the result
    let src_folder_path = snippets_folder_path.join("src");
    let _ = fs::create_dir(src_folder_path);
}

#[must_use]
pub(crate) fn create_and_get_lib_file(snippets_folder_path: &Path, overwrite: bool) -> File {
    let lib_path = snippets_folder_path.join("src").join(LIB_SOURCE_FILE_NAME);
    if overwrite {
        File::create(&lib_path).unwrap()
    } else {
        match File::options().create_new(true).write(true).open(&lib_path) {
            Ok(f) => f,
            Err(_) => {
                println!(
                    "{}",
                    format!(
                        "{lib_path:#?} file already exists, --overwrite option was not provided",
                    )
                    .yellow()
                );
                File::options().write(true).open(&lib_path).unwrap()
            }
        }
    }
}

pub(crate) fn create_main_file(snippets_folder_path: &Path, contract_crate_name: &str) {
    let lib_path = snippets_folder_path
        .join("src")
        .join(SNIPPETS_SOURCE_FILE_NAME);

    let mut file = File::create(lib_path).unwrap();

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

pub(crate) fn create_test_folder_and_get_files(snippets_folder_path: &Path) -> (File, File) {
    let folder_path = snippets_folder_path.join("tests");

    if !Path::new(&folder_path).exists() {
        fs::create_dir_all(&folder_path).expect("Failed to create tests directory");
    }

    let interactor_file_path = folder_path.join(INTERACTOR_TEST_FILE_NAME);
    let interactor_cs_file_path = folder_path.join(INTERACTOR_CS_TEST_FILE_NAME);

    let interactor_file =
        File::create(interactor_file_path).expect("Failed to create interact_tests.rs file");
    let interactor_cs_file =
        File::create(interactor_cs_file_path).expect("Failed to create interact_cs_tests.rs file");

    (interactor_file, interactor_cs_file)
}

pub(crate) fn create_sc_config_file(overwrite: bool, contract_crate_name: &str) {
    let sc_config_path = Path::new("..").join(SC_CONFIG_FILE_NAME);
    let proxy_name = format!("{}_proxy.rs", contract_crate_name.replace("-", "_"),);

    // check if the file should be overwritten or if it already exists
    let mut file = if overwrite || !sc_config_path.exists() {
        File::create(sc_config_path).unwrap()
    } else {
        // file already exists
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(&sc_config_path)
            .unwrap();

        if file_contains_proxy_path(&sc_config_path, &proxy_name).unwrap_or(false) {
            return;
        }

        writeln!(&mut file).unwrap();

        file
    };

    // will be deserialized into a PathBuf, which normalizes the path depending on the platform
    // when deserializing from toml, backwards slashes are not allowed
    let full_proxy_entry = r#"[[proxy]]
path = "interactor/src"#;

    // write full proxy toml entry to the file
    writeln!(&mut file, "{full_proxy_entry}/{proxy_name}\"").unwrap();
}

pub(crate) fn create_config_toml_file(snippets_folder_path: &Path) {
    let config_path = snippets_folder_path.join(CONFIG_TOML_PATH);
    let mut file = File::create(config_path).unwrap();

    writeln!(
        &mut file,
        r#"# chain_type = 'simulator'
# gateway_uri = 'http://localhost:8085'

chain_type = 'real'
gateway_uri = 'https://devnet-gateway.multiversx.com'"#
    )
    .unwrap();
}

pub(crate) fn create_config_rust_file(snippets_folder_path: &Path) -> File {
    let lib_path = snippets_folder_path
        .join("src")
        .join(CONFIG_SOURCE_FILE_NAME);

    File::create(lib_path).unwrap()
}

fn file_contains_proxy_path(file_path: &PathBuf, proxy_name: &str) -> std::io::Result<bool> {
    let file_content = fs::read_to_string(file_path)?;

    let proxy_path = Path::new("interactor").join("src").join(proxy_name);
    let proxy_entry = format!("path = \"{}\"", &proxy_path.to_string_lossy());

    Ok(file_content.contains(&proxy_entry))
}
