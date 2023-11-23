use colored::Colorize;
use std::path::{Path, PathBuf};

/// Finds the workspace by searching for the workspace argument into the project's cargo.
/// Works in debug mode too.
///
pub fn find_current_workspace() -> Option<PathBuf> {
    if let Ok(output) = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
    {
        if let Ok(convert) = std::str::from_utf8(&output.stdout) {
            let path = Path::new(convert.trim());
            if let Some(parent) = path.parent() {
                return Some(parent.to_path_buf());
            }
        }
    }

    None
}

pub fn find_workspace(path: &Path) -> Option<PathBuf> {
    if let Ok(output) = std::process::Command::new(env!("CARGO"))
        .arg(format!("-C {}", path.display()))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
    {
        if let Ok(convert) = std::str::from_utf8(&output.stdout) {
            let path = Path::new(convert.trim());
            if let Some(parent) = path.parent() {
                return Some(parent.to_path_buf());
            }
        }
    }

    None
}

pub fn print_found_workspace(path: &Path) {
    println!(
        "{}",
        format!("Using workspace target directory: {} ...", path.display()).green()
    );
}
