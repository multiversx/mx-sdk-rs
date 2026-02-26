use colored::Colorize;
use multiversx_sc::abi::RustcAbi;
use multiversx_sc_meta_lib::{
    abi_json::deserialize_abi_from_json,
    cli::ContractCliAction,
    tools::{RustcVersion, rustc_version_warning},
};
use semver::Version;

use crate::{cli::AllArgs, folder_structure::RelevantDirectory};
use std::{path::Path, process::Command};

fn should_perform_rustc_version_check(args: &AllArgs) -> bool {
    matches!(args.command, ContractCliAction::Build(_))
}

pub fn verify_rustc_version(contract_crate: &RelevantDirectory, args: &AllArgs) {
    if !should_perform_rustc_version_check(args) {
        return;
    }

    let abi_args = args.to_cargo_abi_for_build();

    let meta_path = contract_crate.meta_path();
    let exit_status = Command::new("cargo")
        .current_dir(&meta_path)
        .args(abi_args)
        .spawn()
        .expect("failed to spawn cargo run process in meta crate")
        .wait()
        .expect("cargo run process in meta crate was not running");

    assert!(exit_status.success(), "contract meta process failed");

    let output_path = contract_crate.output_path();

    assert!(
        output_path.exists(),
        "Output path {} does not exist.",
        output_path.display()
    );
    assert!(
        output_path.is_dir(),
        "Output path {} is not a folder.",
        output_path.display()
    );

    let read_dir = std::fs::read_dir(&output_path).expect("error reading output directory");
    for entry in read_dir {
        let entry = entry.expect("error reading directory entry");
        let path = entry.path();
        if path.is_file()
            && let Some(file_name) = path.file_name().and_then(|s| s.to_str())
            && let Some(contract_name) = file_name.strip_suffix(".abi.json")
        {
            verify_abi_rustc_version(contract_name, &path);
        }
    }
}

fn verify_abi_rustc_version(contract_name: &str, abi_path: &Path) {
    let abi_raw = std::fs::read_to_string(abi_path).expect("error reading ABI file");
    let abi_json = deserialize_abi_from_json(&abi_raw).expect("could not decode ABI");

    let Some(build_info) = abi_json.build_info else {
        return;
    };

    let Some(rustc_abi_json) = build_info.rustc else {
        return;
    };

    let rustc_abi = RustcAbi::from(&rustc_abi_json);
    let framework_version =
        Version::parse(&build_info.framework.version).expect("failed to parse framework version");

    let Some(warning) = rustc_version_warning(
        contract_name,
        framework_version,
        &RustcVersion::from_abi(&rustc_abi),
    ) else {
        return;
    };

    println!("{}", warning.yellow());
}
