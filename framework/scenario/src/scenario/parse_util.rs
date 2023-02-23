use crate::{
    scenario::model::Scenario,
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext},
        serde_raw::ScenarioRaw,
    },
};

use std::{fs, path::Path};

pub fn parse_scenario_raw<P: AsRef<Path>>(path: P) -> ScenarioRaw {
    let contents = fs::read_to_string(path.as_ref())
        .unwrap_or_else(|e| panic!("not found: {} {:?}", e, path.as_ref()));
    serde_json::from_str(contents.as_str()).unwrap()
}

pub fn parse_scenario<P: AsRef<Path>>(path: P) -> Scenario {
    let scenario_parent = path.as_ref().parent().unwrap();
    let interpreter_context = InterpreterContext::new()
        .with_dir(scenario_parent.into())
        .with_allowed_missing_files();
    let raw = parse_scenario_raw(path);
    Scenario::interpret_from(raw, &interpreter_context)
}
