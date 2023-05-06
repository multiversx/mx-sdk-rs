use std::{path::Path, process::Command};

use colored::Colorize;

use crate::{
    cli_args::{AllArgs, CliArgsToRaw},
    folder_structure::{dir_pretty_print, RelevantDirectories},
};

pub fn call_all_meta(args: &AllArgs) {
    let path = if let Some(some_path) = &args.path {
        some_path.as_str()
    } else {
        "./"
    };

    perform_call_all_meta(path, args.ignore.as_slice(), args.to_raw());
}

fn perform_call_all_meta(path: impl AsRef<Path>, ignore: &[String], raw_args: Vec<String>) {
    let dirs = RelevantDirectories::find_all(path, ignore);
    dir_pretty_print(dirs.iter_contract_crates(), "", &|_| {});

    println!(
        "Found {} contract crates.\n",
        dirs.iter_contract_crates().count(),
    );
    if dirs.is_empty() {
        return;
    }

    for contract_crate in dirs.iter_contract_crates() {
        call_contract_meta(contract_crate.path.as_path(), raw_args.as_slice());
    }
}

pub fn call_contract_meta(contract_crate_path: &Path, cargo_run_args: &[String]) {
    let meta_path = contract_crate_path.join("meta");
    assert!(
        meta_path.exists(),
        "Contract meta crate not found at {}",
        meta_path.as_path().display()
    );

    println!(
        "\n{} `cargo run {}` in {}",
        "Calling".green(),
        cargo_run_args.join(" "),
        meta_path.as_path().display(),
    );

    let exit_status = Command::new("cargo")
        .current_dir(&meta_path)
        .args(std::iter::once(&"run".to_string()).chain(cargo_run_args.iter()))
        .spawn()
        .expect("failed to spawn cargo run process in meta crate")
        .wait()
        .expect("cargo run process in meta crate was not running");

    assert!(exit_status.success(), "contract meta process failed");
}
