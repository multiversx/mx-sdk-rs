use std::{fs, path::Path};

use crate::cli::InitConfigArgs;

use super::project_config::CONFIG_FILE_NAME;

const DEFAULT_CONFIG: &str = include_str!("sc-reproducible-build-default.toml");

pub fn init_config(args: &InitConfigArgs) {
    let dir = args
        .path
        .as_deref()
        .map(Path::new)
        .unwrap_or(Path::new("."));

    let dest = dir.join(CONFIG_FILE_NAME);

    if dest.exists() && !args.overwrite {
        eprintln!(
            "Error: {dest} already exists. Use --overwrite to replace it.",
            dest = dest.display()
        );
        std::process::exit(1);
    }

    fs::create_dir_all(dir).unwrap_or_else(|e| {
        eprintln!("Error creating directory {}: {e}", dir.display());
        std::process::exit(1);
    });

    fs::write(&dest, DEFAULT_CONFIG).unwrap_or_else(|e| {
        eprintln!("Error writing {}: {e}", dest.display());
        std::process::exit(1);
    });

    println!("Created: {}", dest.display());
}
