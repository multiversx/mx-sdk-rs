use std::{
    fs::{self, File},
    io::Write,
};

static SNIPPETS_SOURCE_FILE_NAME: &str = "interactor_main.rs";

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

[dependencies.{contract_crate_name}]
path = ".."

[dependencies.multiversx-sc-snippets]
version = "0.50.1"

[dependencies.multiversx-sc]
version = "0.49.0"

[dependencies]
clap = {{ version = "4.4.7", features = ["derive"] }}
serde = {{ version = "1.0", features = ["derive"] }}
toml = "0.8.6"

# [workspace]

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
    let lib_path = format!("{snippets_folder_path}/src/{SNIPPETS_SOURCE_FILE_NAME}");
    if overwrite {
        File::create(&lib_path).unwrap()
    } else {
        match File::options().create_new(true).write(true).open(&lib_path) {
            Ok(f) => f,
            Err(_) => panic!("{lib_path} file already exists, --overwrite option was not provided"),
        }
    }
}

pub(crate) fn create_sc_config_file(overwrite: bool) {
    let sc_config_path = "../sc-config.toml";
    let mut file = if overwrite {
        File::create(sc_config_path).unwrap()
    } else {
        match File::options()
            .create_new(true)
            .write(true)
            .open(sc_config_path)
        {
            Ok(f) => f,
            Err(_) => return,
        }
    };

    writeln!(
        &mut file,
        r#"[[proxy]]
path = "interact-rs/src/proxy.rs"
 "#
    )
    .unwrap();
}
