use std::{
    collections::BTreeSet,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::folder_structure::{RelevantDirectories, RelevantDirectory};

use super::{
    process_code,
    stg_print::*,
    stg_write::{format_test_fn_go, format_test_fn_rs, DEFAULT_SETUP_GO, DEFAULT_SETUP_RS},
    WriteTestFn,
};

const TESTS_DIR_NAME: &str = "tests";
const SCENARIOS_DIR_NAME: &str = "scenarios";

pub fn perform_test_gen_all(path: impl AsRef<Path>, ignore: &[String], create: bool) {
    let root_path = path.as_ref();
    let dirs = RelevantDirectories::find_all(root_path, ignore);

    for contract_dir in dirs.iter() {
        perform_test_gen(contract_dir, create);
    }
}

fn perform_test_gen(contract_dir: &RelevantDirectory, create: bool) {
    let contract_dir_path = &contract_dir.path;
    let scenarios_dir = contract_dir_path.join(SCENARIOS_DIR_NAME);
    if !scenarios_dir.is_dir() {
        print_no_folder(contract_dir_path, SCENARIOS_DIR_NAME);
        return;
    }
    let scenario_names = find_scenario_names(&scenarios_dir);

    let test_dir = contract_dir_path.join(TESTS_DIR_NAME);
    if !test_dir.is_dir() {
        if create {
            fs::create_dir_all(&test_dir).unwrap();
        } else {
            print_no_folder(contract_dir_path, TESTS_DIR_NAME);
            return;
        }
    }

    process_file(
        ProcessFileConfig {
            suffix: "scenario_go_test.rs",
            default_world_impl: DEFAULT_SETUP_GO,
            write_test_fn: format_test_fn_go,
        },
        ProcessFileContext {
            test_dir: &test_dir,
            crate_name: &contract_dir.dir_name_underscores(),
            create_flag: create,
            scenario_names: &scenario_names,
        },
    );

    process_file(
        ProcessFileConfig {
            suffix: "scenario_rs_test.rs",
            default_world_impl: DEFAULT_SETUP_RS,
            write_test_fn: format_test_fn_rs,
        },
        ProcessFileContext {
            test_dir: &test_dir,
            crate_name: &contract_dir.dir_name_underscores(),
            create_flag: create,
            scenario_names: &scenario_names,
        },
    );
}

struct ProcessFileConfig<'a> {
    suffix: &'a str,
    default_world_impl: &'a str,
    write_test_fn: WriteTestFn,
}

struct ProcessFileContext<'a> {
    test_dir: &'a Path,
    crate_name: &'a str,
    create_flag: bool,
    scenario_names: &'a BTreeSet<String>,
}

fn process_file(config: ProcessFileConfig, context: ProcessFileContext) {
    let existing_file_path = find_test_file(context.test_dir, config.suffix);

    if existing_file_path.is_none() && !context.create_flag {
        return;
    }

    let existing_code = if let Some(file_path) = &existing_file_path {
        print_processing(file_path);
        fs::read_to_string(file_path).expect("could not read test file")
    } else {
        config.default_world_impl.to_string()
    };

    let new_code = process_code(
        &existing_code,
        context.scenario_names,
        config.write_test_fn,
        config.default_world_impl,
    );
    let file_path = if let Some(file_path) = &existing_file_path {
        file_path.clone()
    } else {
        let file_name = format!("{}_{}", context.crate_name, config.suffix);
        context.test_dir.join(file_name)
    };
    let mut file = File::create(file_path).unwrap();
    write!(file, "{new_code}").unwrap();
}

fn find_test_file(test_dir: &Path, suffix: &str) -> Option<PathBuf> {
    let read_dir = fs::read_dir(test_dir).expect("error reading directory");
    for file_result in read_dir {
        let file = file_result.unwrap();
        if !file.file_type().unwrap().is_file() {
            continue;
        }
        let file_name = file.file_name().into_string().unwrap();
        if file_name.ends_with(suffix) {
            return Some(file.path());
        }
    }
    None
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
