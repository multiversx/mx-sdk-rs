use crate::{
    cli_args::UpgradeArgs,
    folder_structure::{dir_pretty_print, RelevantDirectories, RelevantDirectory},
    version_history::{versions_iter, LAST_UPGRADE_VERSION, VERSIONS},
};

use super::{
    upgrade_0_31::upgrade_to_31_0,
    upgrade_0_32::upgrade_to_32_0,
    upgrade_0_39::{postprocessing_after_39_0, upgrade_to_39_0},
    upgrade_common::{cargo_check, version_bump_in_cargo_toml},
    upgrade_print::*,
};

pub fn upgrade_sc(args: &UpgradeArgs) {
    let path = if let Some(some_path) = &args.path {
        some_path.as_str()
    } else {
        "./"
    };

    let last_version = args
        .override_target_version
        .clone()
        .unwrap_or_else(|| LAST_UPGRADE_VERSION.to_string());

    assert!(
        VERSIONS.contains(&last_version.as_str()),
        "Invalid requested version: {last_version}",
    );

    let mut dirs = RelevantDirectories::find_all(path, args.ignore.as_slice());
    println!(
        "Found {} directories to upgrade, out of which {} are contract crates.\n",
        dirs.len(),
        dirs.iter_contract_crates().count(),
    );
    dir_pretty_print(dirs.iter(), "", &|dir| {
        print_tree_dir_metadata(dir, last_version.as_str())
    });

    for (from_version, to_version) in versions_iter(last_version) {
        if dirs.count_for_version(from_version) == 0 {
            continue;
        }

        print_upgrading_all(from_version, to_version);
        dirs.start_upgrade(from_version, to_version);
        for dir in dirs.iter_version(from_version) {
            upgrade_function_selector(dir);
        }

        for dir in dirs.iter_version(from_version) {
            upgrade_post_processing(dir);
        }

        // // change the version in memory for the next iteration (dirs is not reloaded from disk)
        // dirs.update_versions_in_memory(from_version, to_version);
        dirs.finish_upgrade();
    }
}

fn upgrade_function_selector(dir: &RelevantDirectory) {
    if dir.upgrade_in_progress.is_some() {
        print_upgrading(dir);
    }

    match dir.upgrade_in_progress {
        Some((_, "0.31.0")) => {
            upgrade_to_31_0(dir);
        },
        Some((_, "0.32.0")) => {
            upgrade_to_32_0(dir);
        },
        Some((_, "0.39.0")) => {
            upgrade_to_39_0(dir);
        },
        Some((from_version, to_version)) => {
            version_bump_in_cargo_toml(&dir.path, from_version, to_version);
        },
        None => {},
    }
}

fn upgrade_post_processing(dir: &RelevantDirectory) {
    match dir.upgrade_in_progress {
        Some((_, "0.28.0")) | Some((_, "0.29.0")) | Some((_, "0.30.0")) | Some((_, "0.31.0"))
        | Some((_, "0.32.0")) | Some((_, "0.33.0")) | Some((_, "0.34.0")) | Some((_, "0.35.0"))
        | Some((_, "0.36.0")) | Some((_, "0.37.0")) | Some((_, "0.40.0")) | Some((_, "0.41.0"))
        | Some((_, "0.42.0")) | Some((_, "0.43.0")) => {
            print_post_processing(dir);
            cargo_check(dir);
        },
        Some((_, "0.39.0")) => {
            print_post_processing(dir);
            postprocessing_after_39_0(dir);
            cargo_check(dir);
        },
        _ => {},
    }
}
