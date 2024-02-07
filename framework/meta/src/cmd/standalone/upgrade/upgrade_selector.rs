use crate::{
    cli_args::UpgradeArgs,
    cmd::standalone::upgrade::upgrade_settings::UpgradeSettings,
    folder_structure::{dir_pretty_print, RelevantDirectories, RelevantDirectory},
    framework_version,
    version::FrameworkVersion,
    version_history::{versions_iter, CHECK_AFTER_UPGRADE_TO, LAST_UPGRADE_VERSION, VERSIONS},
};

use super::{
    upgrade_0_31::upgrade_to_31_0,
    upgrade_0_32::upgrade_to_32_0,
    upgrade_0_39::{postprocessing_after_39_0, upgrade_to_39_0},
    upgrade_0_45::upgrade_to_45_0,
    upgrade_common::{cargo_check, version_bump_in_cargo_toml},
    upgrade_print::*,
};

pub fn upgrade_sc(args: &UpgradeArgs) {
    let path = if let Some(some_path) = &args.path {
        some_path.as_str()
    } else {
        "./"
    };

    let settings = UpgradeSettings::new(args.no_check);

    let last_version = args
        .override_target_version
        .clone()
        .map(|override_target_v| {
            VERSIONS
                .iter()
                .find(|v| v.to_string() == override_target_v)
                .cloned()
                .unwrap_or(LAST_UPGRADE_VERSION)
        })
        .unwrap_or_else(|| LAST_UPGRADE_VERSION);

    assert!(
        VERSIONS.contains(&last_version),
        "Invalid requested version: {}",
        last_version.version,
    );

    let mut dirs = RelevantDirectories::find_all(path, args.ignore.as_slice());
    println!(
        "Found {} directories to upgrade, out of which {} are contract crates.\n",
        dirs.len(),
        dirs.iter_contract_crates().count(),
    );
    dir_pretty_print(dirs.iter(), "", &|dir| {
        print_tree_dir_metadata(dir, &last_version)
    });

    for (from_version, to_version) in versions_iter(last_version) {
        if dirs.count_for_version(from_version) == 0 {
            continue;
        }

        print_upgrading_all(from_version, to_version);
        dirs.start_upgrade(from_version.clone(), to_version.clone());
        for dir in dirs.iter_version(from_version) {
            upgrade_function_selector(dir);
        }

        for dir in dirs.iter_version(from_version) {
            upgrade_post_processing(dir, &settings);
        }

        dirs.finish_upgrade();
    }
}

fn upgrade_function_selector(dir: &RelevantDirectory) {
    if dir.upgrade_in_progress.is_some() {
        print_upgrading(dir);
    }

    if let Some((from_version, to_version)) = &dir.upgrade_in_progress {
        if framework_version!(0.31.0) == *to_version {
            upgrade_to_31_0(dir)
        } else if framework_version!(0.32.0) == *to_version {
            upgrade_to_32_0(dir)
        } else if framework_version!(0.39.0) == *to_version {
            upgrade_to_39_0(dir)
        } else if framework_version!(0.45.0) == *to_version {
            upgrade_to_45_0(dir)
        } else {
            version_bump_in_cargo_toml(&dir.path, from_version, to_version)
        }
    }
}

fn upgrade_post_processing(dir: &RelevantDirectory, settings: &UpgradeSettings) {
    if let Some((_, to_version)) = &dir.upgrade_in_progress {
        if CHECK_AFTER_UPGRADE_TO.contains(to_version) {
            print_post_processing(dir);
            cargo_check(dir, settings);
        } else if framework_version!(0.39.0) == *to_version {
            print_post_processing(dir);
            postprocessing_after_39_0(dir);
            cargo_check(dir, settings);
        }
    }
}
