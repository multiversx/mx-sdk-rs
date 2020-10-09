use super::scenario::*;
use super::scenario_raw::*;
use super::value::InterpretableFrom;
use super::context::*;

use std::fs;
use std::path::Path;

pub fn parse_scenario_raw<P: AsRef<Path>>(path: P) -> ScenarioRaw {
    let contents = fs::read_to_string(path).unwrap();
    serde_json::from_str(contents.as_str()).unwrap()
}

pub fn parse_scenario<P: AsRef<Path>>(path: P) -> Scenario {
    let raw = parse_scenario_raw(path);
    Scenario::interpret_from(raw, &InterpreterContext::default())
}
