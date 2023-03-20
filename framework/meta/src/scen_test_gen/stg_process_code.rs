use std::{
    collections::{BTreeSet, HashMap},
    fs::{self, File},
    io::Write,
    path::Path,
};

use super::{
    stg_parse::parse_section,
    stg_section::{concat_sections, split_sections, Section},
    stg_write::{format_section, WriteTestFn},
};

pub fn process_file(path: &Path, scenario_names: &BTreeSet<String>, write_test_fn: WriteTestFn) {
    let raw_code = fs::read_to_string(path).expect("could not read test file");
    let new_code = process_code(&raw_code, scenario_names, write_test_fn);
    let mut file = File::create(path).unwrap();
    write!(file, "{}", new_code).unwrap();
}

pub fn process_code(
    raw_code: &str,
    scenario_names: &BTreeSet<String>,
    write_test_fn: WriteTestFn,
) -> String {
    let input_sections = split_sections(raw_code);

    let mut result_sections = Vec::new();
    let mut scenario_sections = HashMap::new();

    for mut section in input_sections {
        section.test_fn = parse_section(&section.raw);
        if let Some(scenario_name) = section.scenario_name() {
            scenario_sections.insert(scenario_name, section);
        } else {
            result_sections.push(section);
        }
    }

    for scenario_name in scenario_names {
        let section = scenario_sections
            .remove(scenario_name)
            .unwrap_or_else(|| Section::new_scenario_test(scenario_name));
        result_sections.push(section);
    }

    for section in &mut result_sections {
        if let Some(test_fn) = &section.test_fn {
            section.raw = format_section(test_fn, write_test_fn);
        }
    }

    concat_sections(result_sections.as_slice())
}
