use super::{
    upgrade_0_39::upgrade_39,
    upgrade_common::version_bump_in_cargo_toml,
    upgrade_versions::{iter_from_version, LAST_VERSION},
};
use crate::{
    cli_args::UpgradeArgs,
    folder_structure::{RelevantDirectories, RelevantDirectory},
    sc_upgrade::upgrade_versions::VERSIONS,
};
use colored::*;

pub fn upgrade_sc(args: &UpgradeArgs) {
    let path = if let Some(some_path) = &args.path {
        some_path.as_str()
    } else {
        "./"
    };

    let last_version = args
        .override_target_version
        .clone()
        .unwrap_or_else(|| LAST_VERSION.to_string());

    assert!(
        VERSIONS.contains(&last_version.as_str()),
        "Invalid requested version: {last_version}",
    );

    let dirs = RelevantDirectories::find_all(path);
    println!(
        "Found {} directories to upgrade, out of which {} are contract crates.\n",
        dirs.len(),
        dirs.count_contract_crates(),
    );

    for dir in dirs.iter() {
        if dir.version.semver == last_version {
            print_not_upgrading_ok(dir);
        } else if let Some(iterator) =
            iter_from_version(dir.version.semver.as_str(), Some(last_version.clone()))
        {
            for (from_version, to_version) in iterator {
                print_upgrading(dir, from_version, to_version);
                upgrade_function_selector(dir, from_version, to_version);
            }
        } else {
            print_not_upgrading_unsupported(dir);
        }
    }
}

#[allow(clippy::single_match)] // there will be more than one
fn upgrade_function_selector(dir: &RelevantDirectory, from_version: &str, to_version: &str) {
    match dir.version.semver.as_str() {
        "0.38.0" => {
            upgrade_39(dir);
        },
        _ => {},
    }

    version_bump_in_cargo_toml(&dir.path, from_version, to_version);
}

fn print_upgrading(dir: &RelevantDirectory, from_version: &str, to_version: &str) {
    println!(
        "\n{}",
        format!(
            "Upgrading {} from {} to {}.\n",
            dir.path.display(),
            from_version,
            to_version
        )
        .purple()
    );
}

fn print_not_upgrading_ok(dir: &RelevantDirectory) {
    println!(
        "{}",
        format!(
            "Not upgrading {}, version {} OK.\n",
            dir.path.display(),
            &dir.version.semver
        )
        .green()
    );
}

fn print_not_upgrading_unsupported(dir: &RelevantDirectory) {
    println!(
        "{}",
        format!(
            "Not upgrading {}, version {} unsupported.\n",
            dir.path.display(),
            &dir.version.semver
        )
        .red()
    );
}
