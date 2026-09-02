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
use multiversx_sc_meta_lib::{
    cargo_toml::DependencyReference, cli::ContractCliAction, tools::generate_codehashes_in_output,
    version_history::FIRST_VERSION_WITH_CODEHASH,
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

    let (preprocessed_command, post_processing) =
        preprocess_args(contract_crate, args.command.clone());

    ContractMetaCall::new(preprocessed_command, &args.meta_lib_args)
        .call_for_contract(contract_crate);

    if post_processing.codehash_fallback {
        generate_codehashes_in_output(&contract_crate.output_path());
    }
}

#[derive(Default)]
struct PostProcessing {
    codehash_fallback: bool,
}

/// Prepares the effective command to pass to the contract meta crate and determines
/// whether the codehash should be computed externally by sc-meta after the build.
///
/// When `--codehash` is requested but the contract uses a framework version that predates
/// codehash support, the flag is stripped from the forwarded command and `codehash_fallback`
/// is set to `true` so that sc-meta computes the hash itself afterwards.
fn preprocess_args(
    contract_crate: &RelevantDirectory,
    command: ContractCliAction,
) -> (ContractCliAction, PostProcessing) {
    let mut preprocessed = command;
    let mut post_processing = PostProcessing::default();

    if let ContractCliAction::Build(build_args) = &mut preprocessed {
        if build_args.codehash && should_use_codehash_fallback(contract_crate) {
            build_args.codehash = false;
            post_processing.codehash_fallback = true;
        }
    }

    (preprocessed, post_processing)
}

/// Returns `true` if `--codehash` was requested and the contract's framework version is
/// a plain semver version older than v0.66, meaning the contract meta crate does not
/// support the `--codehash` flag and the hash must be computed by sc-meta instead.
///
/// For non-semver references (path, git, etc.) the flag is forwarded as-is.
fn should_use_codehash_fallback(contract_crate: &RelevantDirectory) -> bool {
    let DependencyReference::Version(version_req) = &contract_crate.version else {
        return false;
    };
    version_req.semver < FIRST_VERSION_WITH_CODEHASH
}
