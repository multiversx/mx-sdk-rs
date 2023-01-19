use std::path::Path;

use colored::Colorize;

use crate::folder_structure::RelevantDirectory;

pub fn print_upgrading(dir: &RelevantDirectory, from_version: &str, to_version: &str) {
    println!(
        "\n{}",
        format!(
            "Upgrading from {from_version} to {to_version} in {}\n",
            dir.path.display(),
        )
        .purple()
    );
}

pub fn print_upgrading_all(from_version: &str, to_version: &str) {
    println!(
        "\n{}",
        format!("Upgrading from {from_version} to {to_version} across crates ...").purple()
    );
}

pub fn print_version_change(
    path: &Path,
    deps_name: &str,
    framework_crate_name: &str,
    from_version: &str,
    to_version: &str,
) {
    println!(
        "{}/{}/{}: {} -> {}",
        path.display(),
        deps_name,
        framework_crate_name.underline(),
        format!("\"{from_version}\"").red().strikethrough(),
        format!("\"{to_version}\"").green()
    )
}

pub fn print_rename(old_path: &Path, new_path: &Path) {
    println!(
        "Renaming {} -> {}",
        old_path.display().to_string().red().strikethrough(),
        new_path.display().to_string().green(),
    );
}

pub fn print_postprocessing_after_39_1(path: &Path) {
    println!(
        "\n{}\n{}",
        format!("Post-processing after 0.39.1 in {} ...", path.display()).green(),
        "Re-generating wasm crate ...".green(),
    );
}
