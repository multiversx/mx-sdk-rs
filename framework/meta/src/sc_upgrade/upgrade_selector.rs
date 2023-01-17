use super::{
    folder_structure::{populate_directories, DirectoryToUpdate},
    upgrade_0_39::upgrade_39,
    upgrade_common::upgrade_cargo_toml_version,
    upgrade_versions::{iter_from_version, LAST_VERSION},
};
use crate::cli_args::UpgradeArgs;
use colored::*;

pub fn upgrade_sc(args: &UpgradeArgs) {
    let path = if let Some(some_path) = &args.path {
        some_path.as_str()
    } else {
        "/home/andreim/elrond/smartcontract/sc-dex-rs"
        // ""
        // "/home/andreim/elrond/smartcontract/sc-nft-marketplace/esdt-nft-marketplace"
    };

    let mut dirs = Vec::new();
    populate_directories(path.as_ref(), &mut dirs);

    for dir in &dirs {
        if dir.version.semver == LAST_VERSION {
            print_not_upgrading_ok(dir);
            println!();
        } else {
            if let Some(iterator) = iter_from_version(dir.version.semver.as_str()) {
                for (from_version, to_version) in iterator {
                    print_upgrading(dir, from_version, to_version);
                    upgrade_function_selector(dir, from_version, to_version);
                }
            } else {
                print_not_upgrading_unsupported(dir);
            }
        }
    }
}

fn upgrade_function_selector(dir: &DirectoryToUpdate, from_version: &str, to_version: &str) {
    match dir.version.semver.as_str() {
        "0.38.0" => {
            upgrade_39(&dir);
        },
        _ => {},
    }

    upgrade_cargo_toml_version(&dir.path, from_version, to_version);
}

fn print_upgrading(dir: &DirectoryToUpdate, from_version: &str, to_version: &str) {
    println!(
        "{}",
        format!(
            "Upgrading {} from {} to {}",
            dir.path.display(),
            from_version,
            to_version
        )
        .purple()
        .underline()
    );
    println!();
}

fn print_not_upgrading_ok(dir: &DirectoryToUpdate) {
    println!(
        "{}",
        format!(
            "Not upgrading {}, version {} OK.",
            dir.path.display(),
            &dir.version.semver
        )
        .green()
    );
    println!();
}

fn print_not_upgrading_unsupported(dir: &DirectoryToUpdate) {
    println!(
        "{}",
        format!(
            "Not upgrading {}, version {} unsupported.",
            dir.path.display(),
            &dir.version.semver
        )
        .red()
    );
    println!();
}
