use std::process::Command;

use colored::Colorize;

use crate::{
    cli_args::{AllArgs, CliArgsToRaw},
    folder_structure::RelevantDirectories,
};

pub fn call_all_meta(args: &AllArgs) {
    let path = if let Some(some_path) = &args.path {
        some_path.as_str()
    } else {
        "./"
    };

    let dirs = RelevantDirectories::find_all(path);
    println!("Found {} contract crates.\n", dirs.count_contract_crates(),);
    if dirs.is_empty() {
        return;
    }

    let mut cargo_run_args = vec!["run".to_string()];
    cargo_run_args.append(&mut args.to_raw());

    for contract_crate in dirs.iter_contract_crates() {
        let meta_path = contract_crate.path.join("meta");

        println!(
            "\n{} `cargo {}` in {}",
            "Calling".green(),
            cargo_run_args.join(" "),
            meta_path.as_path().display(),
        );

        let _ = Command::new("cargo")
            .current_dir(&meta_path)
            .args(cargo_run_args.as_slice())
            .spawn()
            .expect("failed to spawn cargo run process in meta crate")
            .wait()
            .expect("cargo run process in meta crate was not running");
    }
}
