use std::{path::Path, process::Command};

use crate::{
    cli_args::AllArgs,
    folder_structure::{dir_pretty_print, RelevantDirectories},
    print_util::{print_all_command, print_all_count, print_all_index},
};

pub fn call_all_meta(args: &AllArgs) {
    let path = if let Some(some_path) = &args.path {
        some_path.as_str()
    } else {
        "./"
    };

    perform_call_all_meta(path, args.ignore.as_slice(), args.to_cargo_run_args());
}

fn perform_call_all_meta(path: impl AsRef<Path>, ignore: &[String], raw_args: Vec<String>) {
    let dirs = RelevantDirectories::find_all(path, ignore);
    dir_pretty_print(dirs.iter_contract_crates(), "", &|_| {});

    let num_contract_crates = dirs.iter_contract_crates().count();
    print_all_count(num_contract_crates);

    if dirs.is_empty() {
        return;
    }

    for (i, contract_crate) in dirs.iter_contract_crates().enumerate() {
        print_all_index(i + 1, num_contract_crates);
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

    print_all_command(meta_path.as_path(), cargo_run_args);

    let exit_status = Command::new("cargo")
        .current_dir(&meta_path)
        .args(cargo_run_args)
        .spawn()
        .expect("failed to spawn cargo run process in meta crate")
        .wait()
        .expect("cargo run process in meta crate was not running");

    assert!(exit_status.success(), "contract meta process failed");
}
