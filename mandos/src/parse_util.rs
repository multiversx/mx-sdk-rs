use super::context::*;
use super::scenario::*;
use super::scenario_raw::*;
use super::value::InterpretableFrom;

use std::fs;
use std::path::Path;

pub fn parse_scenario_raw<P: AsRef<Path>>(path: P) -> ScenarioRaw {
	let contents = fs::read_to_string(path.as_ref())
		.unwrap_or_else(|e| panic!("not found: {} {:?}", e, path.as_ref()));
	serde_json::from_str(contents.as_str()).unwrap()
}

pub fn parse_scenario<P: AsRef<Path>>(path: P) -> Scenario {
	let raw = parse_scenario_raw(path);
	Scenario::interpret_from(raw, &InterpreterContext::default())
}
