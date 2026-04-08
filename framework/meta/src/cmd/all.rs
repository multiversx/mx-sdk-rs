mod all_rustc_check;

use super::{
    check_wasmer_dependencies::check_wasmer_dependencies,
    print_util::{print_all_command, print_all_count, print_all_index},
};
use crate::{
    cli::AllArgs,
    folder_structure::{RelevantDirectories, dir_pretty_print},
};
use std::{path::Path, process::Command};

pub fn call_all_meta(args: &AllArgs) {
    let path = if let Some(some_path) = &args.path {
        Path::new(some_path)
    } else {
        Path::new("./")
    };

    perform_call_all_meta(path, args);
}

fn perform_call_all_meta(path: &Path, args: &AllArgs) {
    check_wasmer_dependencies(path);

    let dirs = RelevantDirectories::find_all(path, &args.ignore);

    dir_pretty_print(dirs.iter_contract_crates(), "", &|_| {});

    let num_contract_crates = dirs.iter_contract_crates().count();
    print_all_count(num_contract_crates);

    if dirs.is_empty() {
        return;
    }

    for (i, contract_crate) in dirs.iter_contract_crates().enumerate() {
        print_all_index(i + 1, num_contract_crates);

        contract_crate.assert_meta_path_exists();
        let meta_path = contract_crate.meta_path();

        all_rustc_check::verify_rustc_version(contract_crate, args);
        call_contract_meta(&meta_path, &args.to_cargo_run_args());
    }
}

pub fn call_contract_meta(meta_path: &Path, cargo_run_args: &[String]) {
    print_all_command(meta_path, cargo_run_args);

    let exit_status = Command::new("cargo")
        .current_dir(meta_path)
        .args(cargo_run_args)
        .spawn()
        .expect("failed to spawn cargo run process in meta crate")
        .wait()
        .expect("cargo run process in meta crate was not running");

    assert!(exit_status.success(), "contract meta process failed");
}
