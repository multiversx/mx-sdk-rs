use std::{
    collections::{BTreeSet, HashMap},
    fs::{self, File},
    io::Write,
    path::Path,
};

use super::{
    stg_model::Section,
    stg_parse::{parse_section, split_sections},
    stg_write::{format_section, WriteTestFn},
};

pub fn process_file(path: &Path, scenario_names: &BTreeSet<String>, write_test_fn: WriteTestFn) {
    let raw_code = fs::read_to_string(path).expect("could not read test file");
    let input_sections = split_sections(&raw_code);

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

    let mut file = File::create(path).unwrap();
    for section in &result_sections {
        if let Some(test_fn) = &section.test_fn {
            write!(file, "{}", format_section(test_fn, write_test_fn)).unwrap();
        } else {
            writeln!(file, "{}", &section.raw).unwrap();
        }
        for _ in 0..section.num_empty_lines_after {
            writeln!(file).unwrap();
        }
    }
}
