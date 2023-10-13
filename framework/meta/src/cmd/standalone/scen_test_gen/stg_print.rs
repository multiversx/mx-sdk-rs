use std::path::Path;

use colored::Colorize;

pub fn print_no_folder(contract_dir_path: &Path, folder_name: &str) {
    println!(
        "{}",
        format!(
            "No action performed for:   {} (no {folder_name} folder found).",
            contract_dir_path.display(),
        )
        .yellow()
    );
}

pub fn print_processing(test_file_path: &Path) {
    println!(
        "{}",
        format!(
            "Processing scenario tests: {} ...",
            test_file_path.display(),
        )
        .green()
    );
}

pub fn print_no_file(suffix: &str) {
    println!(
        "{}", format!(
            "No file ending in *_{suffix} found. Use the --create flag to create a new {suffix} file.",
        )
        .yellow()
    );
}

pub fn print_new_file(file_path: &Path) {
    println!(
        "{}",
        format!("File {} has been created", file_path.display()).green()
    );
}
