use std::{
    fs::{self, File},
    io::Write,
};

pub(crate) fn create_snippets_folder(snippets_folder_path: &str) {
    // returns error if folder already exists, so we ignore the result
    let _ = fs::create_dir(snippets_folder_path);
}

pub(crate) fn create_snippets_gitignore(snippets_folder_path: &str, overwrite: bool) {
    let gitignore_path = format!("{}/.gitignore", snippets_folder_path);
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
    let cargo_toml_path = format!("{}/Cargo.toml", snippets_folder_path);
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
edition = "2018"
publish = false

[[bin]]
name = "rust-interact"
path = "src/lib.rs"

[dependencies.{}]
path = ".."

[dependencies.elrond-interact-snippets]
version = "0.1.0"

[workspace]

"#,
        contract_crate_name
    )
    .unwrap();
}

pub(crate) fn create_src_folder(snippets_folder_path: &str) {
    // returns error if folder already exists, so we ignore the result
    let src_folder_path = format!("{}/src", snippets_folder_path);
    let _ = fs::create_dir(src_folder_path);
}

#[must_use]
pub(crate) fn create_and_get_lib_file(snippets_folder_path: &str, overwrite: bool) -> File {
    let lib_path = format!("{}/src/lib.rs", snippets_folder_path);
    if overwrite {
        File::create(&lib_path).unwrap()
    } else {
        match File::options().create_new(true).write(true).open(&lib_path) {
            Ok(f) => f,
            Err(_) => panic!("lib.rs file already exists, --overwrite option was not provided"),
        }
    }
}
