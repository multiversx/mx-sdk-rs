use std::{
    fs::{read_dir, read_to_string, ReadDir},
    path::{Path, PathBuf},
};

use toml::Value;

const CHAIN_TYPE: &str = "chain_type";
const SIMULATOR: &str = "simulator";
const CONFIG: &str = "config.toml";

pub(crate) fn simulator_setup(interactors_dir: &Path) -> bool {
    let read_dir = read_dir(interactors_dir).expect("Error reading directory");
    let config_file_path = match extract_path_config_toml(read_dir) {
        Some(path_config) => path_config,
        None => return false,
    };

    is_chain_simulator_config(config_file_path)
}

pub(crate) fn is_chain_simulator_config(config_path: PathBuf) -> bool {
    let config_content = read_to_string(config_path).expect("Failed to read configuration file.");

    let parsed_config = config_content
        .parse::<Value>()
        .expect("Failed to parse TOML content.");

    if let Some(chain_type) = parsed_config.get(CHAIN_TYPE).and_then(Value::as_str) {
        return chain_type.eq(SIMULATOR);
    }

    false
}

pub(crate) fn extract_path_config_toml(read_dir: ReadDir) -> Option<PathBuf> {
    for file_result in read_dir.into_iter() {
        let file = file_result.unwrap();
        if !file.file_type().unwrap().is_file() {
            continue;
        }
        if file.path().ends_with(CONFIG) {
            return Some(file.path());
        }
    }

    None
}
