use std::path::{Path, PathBuf};

use colored::Colorize;

/// Finds the workspace by taking the `current_exe` and working its way up.
/// Works in debug mode too.
///
pub fn find_workspace() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    print_found_workspace(cargo_path.parent().unwrap());
    cargo_path.parent().unwrap().to_path_buf()
}

pub fn print_searching_for_workspace() {
    println!(
        "{}",
        format!("No --target-dir specified. Searching for workspace ...").yellow()
    );
}

pub fn print_found_workspace(path: &Path) {
    println!(
        "{}",
        format!("Workspace found: {} ...", path.display()).green()
    );
}
