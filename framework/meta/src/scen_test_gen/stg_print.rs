use std::path::Path;

use colored::Colorize;

pub fn print_no_folder(contract_dir_path: &Path, folder_name: &str) {
    println!(
        "{}",
        format!(
            "Warning: no {folder_name} folder found under {}, no action performed.",
            contract_dir_path.display(),
        )
        .yellow()
    );
}
