use std::collections::{BTreeSet, HashMap};

use super::{
    stg_parse::parse_section,
    stg_section::{concat_sections, split_sections, Section},
    stg_write::{contains_world_fn, format_section, WriteTestFn},
};

pub fn process_code(
    raw_code: &str,
    scenario_names: &BTreeSet<String>,
    write_test_fn: WriteTestFn,
    default_setup_code: &str,
) -> String {
    let input_sections = split_sections(raw_code);

    let mut result_sections = Vec::new();
    let mut scenario_sections = HashMap::new();

    if !contains_world_fn(raw_code) {
        result_sections.push(Section {
            raw: default_setup_code.to_string(),
            num_empty_lines_after: 1,
            test_fn: None,
        });
    }

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
