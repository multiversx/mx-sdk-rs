use super::stg_section::ScenarioTestFn;

pub const TEST_ANNOTATION: &str = "#[test]";
pub const IGNORE_ANNOTATION: &str = "#[ignore]";
pub const IGNORE_ANNOTATION_PREFIX: &str = "#[ignore";
pub const SCEN_PATTERN_PREFIX: &str = "\"scenarios/";
pub const SCEN_PATTERN_SUFFIX: &str = ".scen.json\"";

pub fn parse_section(section_str: &str) -> Option<ScenarioTestFn> {
    let mut docs = String::new();
    let mut ignore_line = None;
    let mut opt_test_line = None;
    let mut all_commented_out = true;
    let mut opt_scenario_file_name = None;

    for line in section_str.lines() {
        // extract docs
        if opt_test_line.is_none() && line.starts_with("//") {
            docs.push_str(line);
            docs.push('\n');
        }

        // one non-commented-out line is enough to set flag to false
        if !line.starts_with("//") {
            all_commented_out = false;
        }

        let uncomm_line = line.strip_prefix("// ").unwrap_or(line);
        if uncomm_line.starts_with(TEST_ANNOTATION) {
            opt_test_line = Some(uncomm_line.to_string());
        } else if uncomm_line.starts_with(IGNORE_ANNOTATION_PREFIX) {
            ignore_line = Some(uncomm_line.to_string());
        } else if let Some(scenario_file_name) = find_scenario_name(uncomm_line) {
            opt_scenario_file_name = Some(scenario_file_name.to_string());
        }
    }

    // functions should not be commented out, but ignored
    if all_commented_out && ignore_line.is_none() {
        ignore_line = Some(IGNORE_ANNOTATION.to_string());
    }
    if let Some(first_line) = section_str.lines().next() {
        if let Some(comment) = first_line.strip_prefix("/*") {
            ignore_line = Some(format!(
                "{IGNORE_ANNOTATION_PREFIX} = \"{}\"]",
                comment.trim()
            ));
        }
    }

    if let (Some(test_line), Some(scenario_file_name)) = (opt_test_line, opt_scenario_file_name) {
        Some(ScenarioTestFn {
            docs,
            test_line,
            ignore_line,
            scenario_file_name,
        })
    } else {
        None
    }
}

/// Extracts a pattern of the form `"scenarios/<result>.scen.json"`.
///
/// Could be done with regex, but this is more lightweight, and good enough in here.
fn find_scenario_name(s: &str) -> Option<&str> {
    if let Some(prefix_index) = s.find(SCEN_PATTERN_PREFIX) {
        if let Some(suffix_index) = s.find(SCEN_PATTERN_SUFFIX) {
            return s.get(prefix_index + SCEN_PATTERN_PREFIX.len()..suffix_index);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_scenario_name() {
        assert_eq!(
            find_scenario_name(r#"  "scenarios/test_name.scen.json"  "#),
            Some("test_name")
        );
        assert_eq!(
            find_scenario_name(r#"f("scenarios/test_name.scen.json");"#),
            Some("test_name")
        );
        assert_eq!(
            find_scenario_name(r#"first: ".scen.json" then: "scenarios/ ..."#),
            None
        );
    }
}
