use std::{
    fs::{self, File},
    io::Write,
};

use super::meta_config::MetaConfig;

impl MetaConfig {
    // TODO: Handle overwrite flag
    pub fn generate_rust_snippets(&self) {
        if let Some(contract) = &self.main_contract {
            let crate_name = contract.output_base_name.clone();
            let _ = create_snippets_crate_and_get_lib_file(&self.snippets_dir, &crate_name, true);
        }
    }
}

#[must_use]
fn create_snippets_crate_and_get_lib_file(
    snippets_folder_path: &str,
    contract_crate_name: &str,
    overwrite: bool,
) -> File {
    create_snippets_folder(snippets_folder_path);
    create_snippets_cargo_toml(snippets_folder_path, contract_crate_name, overwrite);
    create_src_folder(snippets_folder_path);
    create_and_get_lib_file(snippets_folder_path, overwrite)
}

fn create_snippets_folder(snippets_folder_path: &str) {
    // returns error if folder already exists, so we ignore the result
    let _ = fs::create_dir(snippets_folder_path);
}

fn create_snippets_cargo_toml(
    snippets_folder_path: &str,
    contract_crate_name: &str,
    overwrite: bool,
) {
    let cargo_toml_path = format!("{}/Cargo.toml", snippets_folder_path);
    let mut file = if overwrite {
        File::create(&cargo_toml_path).unwrap()
    } else {
        match File::options().create_new(true).open(&cargo_toml_path) {
            Ok(f) => f,
            Err(_) => return,
        }
    };

    file.write_fmt(format_args!(
        "[package]
name = \"rust-interact\"
version = \"0.0.0\"
authors = [\"you\"]
edition = \"2018\"
publish = false

[[bin]]
name = \"rust-interact\"
path = \"src/lib.rs\"

[dependencies.{}]
path = \"..\"

[dependencies.elrond-interact-snippets]
version = \"0.1.0\"
",
        contract_crate_name
    ))
    .unwrap();
}

fn create_src_folder(snippets_folder_path: &str) {
    // returns error if folder already exists, so we ignore the result
    let src_folder_path = format!("{}/src", snippets_folder_path);
    let _ = fs::create_dir(src_folder_path);
}

#[must_use]
fn create_and_get_lib_file(snippets_folder_path: &str, overwrite: bool) -> File {
    let lib_path = format!("{}/src/lib.rs", snippets_folder_path);
    if overwrite {
        File::create(&lib_path).unwrap()
    } else {
        match File::options().create_new(true).open(&lib_path) {
            Ok(f) => f,
            Err(_) => panic!("lib.rs file already exists, overwrite option was not provided"),
        }
    }
}
