use std::{
    fs::{read_dir, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

const SOURCE: &str = "src";
const CHAIN_FEATURE: &str = r#"#[cfg(feature = "chain_simulator")]"#;

pub(crate) fn interactor_contains_simulator_feature(interactors_dir: &Path) -> bool {
    let interactors_src_dir = interactors_dir.join(SOURCE);
    let src_dir = read_dir(interactors_src_dir).expect("Error reading directory");
    for scr_file_result in src_dir {
        let file = scr_file_result.unwrap();
        if !file.file_type().unwrap().is_file() {
            continue;
        }

        if contains_simulator_feature_uncommented(&file.path()) {
            return true;
        }
    }
    false
}

fn contains_simulator_feature_uncommented(file_path: &PathBuf) -> bool {
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut in_block_comment = false;

    for line in reader.lines() {
        let line = line.expect("Unable to parse line");
        let trimmed = line.trim();

        if trimmed.contains("/*") {
            in_block_comment = true;
        }

        if trimmed.contains("*/") {
            in_block_comment = false;
            continue;
        }

        if in_block_comment {
            continue;
        }

        if !trimmed.starts_with("//") && trimmed.contains(CHAIN_FEATURE) {
            return true;
        }
    }

    false
}
