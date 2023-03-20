use std::{collections::BTreeSet, fs, path::Path};

use crate::{
    folder_structure::RelevantDirectories,
    scen_test_gen::stg_write::{format_test_fn_go, format_test_fn_rs},
};

use super::{stg_print::print_no_folder, stg_process_code::process_file};

const TESTS_DIR_NAME: &str = "tests";
const SCENARIOS_DIR_NAME: &str = "scenarios";

pub fn perform_test_gen_all(path: impl AsRef<Path>, ignore: &[String]) {
    let root_path = path.as_ref();
    let dirs = RelevantDirectories::find_all(root_path, ignore);

    for contract_dir in dirs.iter_contract_crates() {
        perform_test_gen(&contract_dir.path);
    }
}

fn perform_test_gen(contract_dir_path: &Path) {
    let test_dir = contract_dir_path.join(TESTS_DIR_NAME);
    if !test_dir.is_dir() {
        print_no_folder(contract_dir_path, TESTS_DIR_NAME);
        return;
    }

    let scenarios_dir = contract_dir_path.join(SCENARIOS_DIR_NAME);
    if !scenarios_dir.is_dir() {
        print_no_folder(contract_dir_path, SCENARIOS_DIR_NAME);
        return;
    }
    let scenario_names = find_scenario_names(&scenarios_dir);

    let read_dir = fs::read_dir(test_dir).expect("error reading directory");
    for file_result in read_dir {
        let file = file_result.unwrap();
        if !file.file_type().unwrap().is_file() {
            continue;
        }
        let file_name = file.file_name().into_string().unwrap();
        if file_name.ends_with("scenario_rs_test.rs") {
            process_file(&file.path(), &scenario_names, format_test_fn_rs);
        }
        if file_name.ends_with("scenario_go_test.rs") {
            process_file(&file.path(), &scenario_names, format_test_fn_go);
        }
    }
}

fn find_scenario_names(scenarios_dir: &Path) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    let read_dir = fs::read_dir(scenarios_dir).expect("error reading directory");
    for file_result in read_dir {
        let file = file_result.unwrap();
        if !file.file_type().unwrap().is_file() {
            continue;
        }
        let file_name = file.file_name().into_string().unwrap();
        if let Some(scenario_name) = file_name.strip_suffix(".scen.json") {
            result.insert(scenario_name.to_string());
        }
    }
    result
}
