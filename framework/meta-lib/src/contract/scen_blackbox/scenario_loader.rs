use multiversx_chain_scenario_format::serde_raw::ScenarioRaw;
use std::{
    fs,
    path::Path,
};

/// Represents a parsed scenario file
pub struct ScenarioFile {
    pub file_name: String,
    pub scenario: ScenarioRaw,
}

/// Scans the scenarios folder and loads all .scen.json files
pub fn load_scenario_files(scenarios_dir: &Path) -> Vec<ScenarioFile> {
    if !scenarios_dir.exists() {
        return Vec::new();
    }

    let mut scenario_files = Vec::new();

    if let Ok(entries) = fs::read_dir(scenarios_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "json"
                        && path
                            .file_name()
                            .and_then(|s| s.to_str())
                            .map(|s| s.ends_with(".scen.json"))
                            .unwrap_or(false)
                    {
                        if let Some(scenario_file) = load_scenario_file(&path) {
                            scenario_files.push(scenario_file);
                        }
                    }
                }
            }
        }
    }

    scenario_files
}

/// Loads a single scenario file
fn load_scenario_file(path: &Path) -> Option<ScenarioFile> {
    let file_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())?;

    let scenario = ScenarioRaw::load_from_file(path);

    Some(ScenarioFile {
        file_name,
        scenario,
    })
}

/// Converts a scenario file name to a valid Rust function name
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
        assert_eq!(scenario_to_function_name("test-case.with-dots"), "test_case_with_dots");
        assert_eq!(scenario_to_function_name("my-test.scen"), "my_test_scen");
        assert_eq!(scenario_to_function_name("a-b-c.d.e"), "a_b_c_d_e");
    }
}
