use colored::Colorize;
use multiversx_sc_meta_lib::{cargo_toml::DependencyReference, version::FrameworkVersion};
use std::path::Path;

use crate::folder_structure::{DirectoryType, RelevantDirectory};

pub fn print_all_count(num_contract_crates: usize) {
    println!(
        "\n{}",
        format!("Found {num_contract_crates} contract crates.").truecolor(128, 128, 128),
    );
}

pub fn print_all_index(contract_crates_index: usize, num_contract_crates: usize) {
    println!(
        "\n{}",
        format!("({contract_crates_index}/{num_contract_crates})").truecolor(128, 128, 128),
    );
}

pub fn print_all_command(meta_path: &Path, cargo_run_args: &[String]) {
    println!(
        "{} {}\n{} `cargo {}`",
        "In".green(),
        meta_path.display(),
        "Calling".green(),
        cargo_run_args.join(" "),
    );
}

pub fn print_tree_dir_metadata(dir: &RelevantDirectory, last_version: &FrameworkVersion) {
    match dir.dir_type {
        DirectoryType::Contract => print!(" {}", "[contract]".blue()),
        DirectoryType::Lib => print!(" {}", "[lib]".magenta()),
    }

    match &dir.version {
        DependencyReference::Version(version_req) => {
            let version_string = format!("[{}]", version_req.semver);
            if version_req.semver == *last_version {
                print!(" {}", version_string.green());
            } else {
                print!(" {}", version_string.red());
            };
        },
        DependencyReference::GitCommit(git_reference) => {
            let git_string = format!(
                "[git: {} rev: {}]",
                git_reference.git.truecolor(255, 127, 0),
                git_reference.rev.truecolor(255, 127, 0)
            );
            print!(" {}", git_string.truecolor(255, 198, 0));
        },
        DependencyReference::GitBranch(git_reference) => {
            let git_string = format!(
                "[git: {} branch: {}]",
                git_reference.git.truecolor(255, 127, 0),
                git_reference.branch.truecolor(255, 127, 0)
            );
            print!(" {}", git_string.truecolor(255, 198, 0));
        },
        DependencyReference::GitTag(git_reference) => {
            let git_string = format!(
                "[git: {} rev: {}]",
                git_reference.git.truecolor(255, 127, 0),
                git_reference.tag.truecolor(255, 127, 0)
            );
            print!(" {}", git_string.truecolor(255, 198, 0));
        },
        DependencyReference::Path(path_buf) => {
            let git_string = format!(
                "[path: {}]",
                path_buf.to_string_lossy().truecolor(255, 127, 0)
            );
            print!(" {}", git_string.truecolor(255, 198, 0));
        },
        DependencyReference::Unsupported(error) => {
            let message = format!("dependency error: {error}");
            print!(" {}", message.red());
        },
    }
}
