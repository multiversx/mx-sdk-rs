use super::{folder_structure::populate_directories, upgrade_0_39::upgrade_39};
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
        match dir.version.semver.as_str() {
            "0.38.0" => {
                println!(
                    "{}",
                    format!("Upgrading {} from 0.38.0 to 0.39.0", dir.path.display())
                        .purple()
                        .underline()
                );
                println!();
                upgrade_39(&dir);
            },
            _ => {
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
            },
        }
    }

    // let contract_paths = find_contract_paths(sc_crate_path.as_ref());
    // for contract_path in contract_paths {
    //     println!("Upgrading: {}", contract_path.display());

    //     let version = find_framework_version(contract_path.as_ref());
    //     println!("Current version: {}", version);

    //     match version.as_str() {
    //         "0.38.0" | "=0.38.0" => {
    //             upgrade_39(contract_path.as_ref());
    //         },
    //         _ => {
    //             println!("Unsupported version.");
    //         },
    //     }
    // }
}
