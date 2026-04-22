mod all_rustc_check;
mod contract_meta_call;

pub use contract_meta_call::ContractMetaCall;

use super::{
    check_wasmer_dependencies::check_wasmer_dependencies,
    print_util::{print_all_count, print_all_index},
};
use crate::{
    cli::AllArgs,
    folder_structure::{RelevantDirectories, RelevantDirectory, dir_pretty_print},
};
use std::path::Path;

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

    dirs.warn_duplicate_contract_names();

    for (i, contract_crate) in dirs.iter_contract_crates().enumerate() {
        print_all_index(i + 1, num_contract_crates);
        call_contract_meta(contract_crate, args);
    }
}

pub fn call_contract_meta(contract_crate: &RelevantDirectory, args: &AllArgs) {
    contract_crate.assert_meta_path_exists();
    all_rustc_check::verify_rustc_version(contract_crate, args);
    ContractMetaCall::new(args.command.clone(), &args.meta_lib_args)
        .call_for_contract(contract_crate);
}
