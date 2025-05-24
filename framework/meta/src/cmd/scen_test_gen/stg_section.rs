use super::stg_parse::TEST_ANNOTATION;

/// Helps splitting the test file and handling whitespace.
#[derive(Default)]
pub struct Section {
    /// Note: does not contain a trailing newline, normally ends with '}'.
    pub raw: String,
    pub num_empty_lines_after: usize,
    pub test_fn: Option<ScenarioTestFn>,
}

/// Parsed secion.
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
        self.test_fn
            .as_ref()
            .map(|test_fn| test_fn.scenario_file_name.clone())
    }
}

pub fn split_sections(s: &str) -> Vec<Section> {
    let mut result = Vec::new();
    let mut is_within_section = true;
    let mut current_section = Section::default();

    if s.is_empty() {
        return result;
    }

    for line in s.lines() {
        if is_within_section {
            current_section.raw.push_str(line);
            if line == "}" {
                is_within_section = false;
            } else {
                current_section.raw.push('\n');
            }
        } else if str_is_whitespace(line) {
            current_section.num_empty_lines_after += 1;
        } else {
            result.push(std::mem::take(&mut current_section));
            is_within_section = true;
            current_section.raw.push_str(line);
            current_section.raw.push('\n');
        }
    }
    current_section.num_empty_lines_after += 1;
    result.push(current_section);
    result
}

fn str_is_whitespace(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_whitespace() {
            return false;
        }
    }
    true
}

pub fn concat_sections(sections: &[Section]) -> String {
    let mut assembled = String::new();
    for (index, section) in sections.iter().enumerate() {
        assembled.push_str(&section.raw);
        assembled.push('\n');
        let last = index == sections.len() - 1;
        let num_empty_lines_after = if last && section.num_empty_lines_after > 0 {
            // the final newline in the file
            section.num_empty_lines_after - 1
        } else {
            section.num_empty_lines_after
        };
        for _ in 0..num_empty_lines_after {
            assembled.push('\n');
        }
    }
    assembled
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_whitespace_1() {
        const SECTIONS: &str = r#"fn section1() {

}
"#;
        let sections = split_sections(SECTIONS);
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].num_empty_lines_after, 1);
        assert_eq!(SECTIONS.to_string(), concat_sections(&sections));
    }

    #[test]
    fn test_section_whitespace_2() {
        const SECTIONS: &str = r#"fn section1() {
}
fn section2() {
}
"#;
        let sections = split_sections(SECTIONS);
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].num_empty_lines_after, 0);
        assert_eq!(sections[1].num_empty_lines_after, 1);
        assert_eq!(SECTIONS.to_string(), concat_sections(&sections));
    }

    #[test]
    fn test_split_sections() {
        const INPUT: &str = r#"use something;

fn another_func() {
    // ...
}
          


#[test]
fn test_1() {
    multiversx_sc_scenario::run_rs("scenarios/test1.scen.json");
}
  
#[test]
fn test_2() {
    multiversx_sc_scenario::run_rs("scenarios/test2.scen.json");
}

"#;
        let sections = split_sections(INPUT);
        assert_eq!(sections.len(), 3);
        assert_eq!(sections[0].num_empty_lines_after, 3);
        assert_eq!(sections[1].num_empty_lines_after, 1);
        assert_eq!(sections[2].num_empty_lines_after, 2);
    }
}
