use crate::{
    cli_args::UpgradeArgs,
    folder_structure::{RelevantDirectories, RelevantDirectory},
    sc_upgrade::{
        upgrade_0_39::{postprocessing_after_39_1, upgrade_to_39_0},
        upgrade_common::version_bump_in_cargo_toml,
        upgrade_print::*,
        upgrade_versions::{versions_iter, DEFAULT_LAST_VERSION, VERSIONS},
    },
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
        .unwrap_or_else(|| DEFAULT_LAST_VERSION.to_string());

    assert!(
        VERSIONS.contains(&last_version.as_str()),
        "Invalid requested version: {last_version}",
    );

    let mut dirs = RelevantDirectories::find_all(path);
    println!(
        "Found {} directories to upgrade, out of which {} are contract crates.\n",
        dirs.len(),
        dirs.iter_contract_crates().count(),
    );

    for (from_version, to_version) in versions_iter(last_version) {
        if dirs.count_for_version(from_version) == 0 {
            continue;
        }

        print_upgrading_all(from_version, to_version);
        for dir in dirs.iter_version(from_version) {
            print_upgrading(dir, from_version, to_version);
            upgrade_function_selector(dir, from_version, to_version);
        }

        for dir in dirs.iter_version(from_version) {
            upgrade_post_processing(dir);
        }

        // change the version in memory for the next iteration (dirs is not reloaded from disk)
        dirs.update_versions_in_memory(from_version, to_version);
    }
}

fn upgrade_function_selector(dir: &RelevantDirectory, from_version: &str, to_version: &str) {
    #[allow(clippy::single_match)]
    match dir.version.semver.as_str() {
        "0.38.0" => {
            upgrade_to_39_0(dir);
        },
        _ => {
            version_bump_in_cargo_toml(&dir.path, from_version, to_version);
        },
    }
}

fn upgrade_post_processing(dir: &RelevantDirectory) {
    #[allow(clippy::single_match)]
    match dir.version.semver.as_str() {
        "0.39.0" => {
            postprocessing_after_39_1(dir);
        },
        _ => {},
    }
}
