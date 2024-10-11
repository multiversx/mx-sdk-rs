use std::path::{Path, PathBuf};

use colored::Colorize;

use crate::folder_structure::{RelevantDirectories, RelevantDirectory};

use super::{
    execute_and_print, interactor_test_analyzer::interactor_contains_simulator_feature,
    simulator_setup::simulator_setup,
};

const INTERACT: &str = "interact";

pub(crate) fn perform_tests_interactor(path: impl AsRef<Path>) {
    let root_path = path.as_ref();

    let dirs = RelevantDirectories::find_all(root_path, &[]);

    for contract_dir in dirs.iter() {
        perform_test_interactor_for_contract(contract_dir);
    }
}

pub(crate) fn perform_test_interactor_for_contract(contract_dir: &RelevantDirectory) {
    let contract_dir_path = &contract_dir.path;

    let interactor_dir = match find_interactor_dir(contract_dir_path) {
        Some(interactor_directory) => interactor_directory,
        None => return,
    };

    if simulator_setup(&interactor_dir) {
        execute_and_print(
            interactor_dir.to_str().unwrap(),
            "cargo",
            &["test", "--features", "chain_simulator"],
        );
    } else if interactor_contains_simulator_feature(&interactor_dir) {
        panic!(
            "{}",
            format!(
                "For path {} you have to set chain type as simulator",
                interactor_dir.as_path().display()
            )
            .bright_red(),
        )
    }
}

fn find_interactor_dir(contract_dir_path: &Path) -> Option<PathBuf> {
    if contract_dir_path.ends_with(INTERACT) {
        return Some(contract_dir_path.to_path_buf());
    }

    let interactor_dir = contract_dir_path.join(INTERACT);
    if !interactor_dir.exists() {
        return None;
    }
    if !interactor_dir.is_dir() {
        print_no_folder(contract_dir_path, INTERACT);
        return None;
    }

    Some(interactor_dir)
}

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
