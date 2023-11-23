use colored::Colorize;
use std::path::{Path, PathBuf};

/// Finds the workspace by searching for the workspace argument into the project's cargo.
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
    let to_show = "No --target-dir specified. Searching for workspace ..."
        .to_string()
        .yellow();
    println!("{to_show}");
}

pub fn print_found_workspace(path: &Path) {
    println!(
        "{}",
        format!("Workspace found: {} ...", path.display()).green()
    );
}
