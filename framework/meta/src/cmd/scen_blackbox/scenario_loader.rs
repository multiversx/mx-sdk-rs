use multiversx_sc_scenario::scenario::model::Scenario;
use multiversx_sc_scenario::scenario::parse_scenario;
use std::{fs, path::Path};

/// Represents a parsed scenario file
pub struct ScenarioFile {
    pub file_name: String,
    pub scenario: Scenario,
    /// If true, a `#[test]` wrapper function is generated in addition to the steps function.
    /// `.scen.json` files generate a test; `.steps.json` files only generate a steps function.
    pub generate_test: bool,
}

impl ScenarioFile {
    /// The base Rust identifier derived from the file name.
    /// Used as the `#[test]` function name for `.scen.json` files.
    pub fn test_name(&self) -> String {
        scenario_to_function_name(&self.file_name)
    }

    /// The name of the generated steps function.
    /// `.scen.json` files → `{test_name}_steps`
    /// `.steps.json` files → `{test_name}` (avoids `_steps_steps`)
    pub fn steps_function_name(&self) -> String {
        Self::steps_function_name_of(&self.file_name, self.generate_test)
    }

    /// Associated function variant – usable without a `ScenarioFile` instance.
    pub fn steps_function_name_of(file_name: &str, generate_test: bool) -> String {
        let base = scenario_to_function_name(file_name);
        if generate_test {
            format!("{}_steps", base)
        } else {
            base
        }
    }
}

/// Scans the scenarios folder and loads all .scen.json and .steps.json files,
/// recursing into sub-folders.
pub fn load_scenario_files(scenarios_dir: &Path) -> Vec<ScenarioFile> {
    if !scenarios_dir.exists() {
        return Vec::new();
    }

    let mut scenario_files = Vec::new();
    load_scenario_files_recursive(scenarios_dir, &mut scenario_files);
    scenario_files
}

fn load_scenario_files_recursive(current_dir: &Path, scenario_files: &mut Vec<ScenarioFile>) {
    let Ok(entries) = fs::read_dir(current_dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            load_scenario_files_recursive(&path, scenario_files);
        } else if path.is_file() {
            if let Some(file_name_str) = path.file_name().and_then(|s| s.to_str()) {
                if file_name_str.ends_with(".scen.json") {
                    if let Some(scenario_file) = load_scenario_file(&path, true) {
                        scenario_files.push(scenario_file);
                    }
                } else if file_name_str.ends_with(".steps.json") {
                    if let Some(scenario_file) = load_scenario_file(&path, false) {
                        scenario_files.push(scenario_file);
                    }
                }
            }
        }
    }
}

/// Loads a single scenario file.
/// The `file_name` is always the file stem (no sub-folder prefix),
/// matching the name used at the call site for function generation.
fn load_scenario_file(path: &Path, generate_test: bool) -> Option<ScenarioFile> {
    let file_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())?;

    let scenario = parse_scenario(path);

    Some(ScenarioFile {
        file_name,
        scenario,
        generate_test,
    })
}

/// Converts a scenario file stem to a valid Rust identifier.
/// Replaces `-` and `.` with `_`.
pub fn scenario_to_function_name(scenario_name: &str) -> String {
    scenario_name.replace(['-', '.'], "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_to_function_name() {
        assert_eq!(scenario_to_function_name("simple"), "simple");
        assert_eq!(scenario_to_function_name("test-case"), "test_case");
        assert_eq!(scenario_to_function_name("test.case"), "test_case");
        assert_eq!(
            scenario_to_function_name("test-case.with-dots"),
            "test_case_with_dots"
        );
        assert_eq!(scenario_to_function_name("my-test.scen"), "my_test_scen");
        assert_eq!(scenario_to_function_name("a-b-c.d.e"), "a_b_c_d_e");
    }
}
