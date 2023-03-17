use super::stg_parse::TEST_ANNOTATION;

#[derive(Default)]
pub struct Section {
    pub raw: String,
    pub num_empty_lines_after: usize,
    pub test_fn: Option<ScenarioTestFn>,
}

pub struct ScenarioTestFn {
    pub docs: String,
    pub test_line: String,
    pub ignore_line: Option<String>,
    pub scenario_file_name: String,
}

impl Section {
    pub fn new_scenario_test(scenario_name: &str) -> Self {
        Section {
            raw: String::new(),
            num_empty_lines_after: 1,
            test_fn: Some(ScenarioTestFn {
                docs: String::new(),
                ignore_line: None,
                test_line: TEST_ANNOTATION.to_string(),
                scenario_file_name: scenario_name.to_string(),
            }),
        }
    }

    pub fn scenario_name(&self) -> Option<String> {
        if let Some(test_fn) = &self.test_fn {
            Some(test_fn.scenario_file_name.clone())
        } else {
            None
        }
    }
}
