use std::{fs, fs::File, io::Write};

use elrond_wasm_debug::mandos_system::model::Scenario;

use mandos::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::ScenarioRaw,
};

#[test]
fn test_scenario_low_to_low_ser_de() {
    let contents = fs::read_to_string("./tests/mandos-io/example_raw.scen.json").unwrap();
    let scenario_raw = ScenarioRaw::from_json_str(contents.as_str());

    let serialized = scenario_raw.to_json_string();

    let mut file = File::create("tests/mandos-io/serialized_raw.scen.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    assert_eq!(serialized, contents);
}

#[test]
fn test_scenario_low_to_high_ser_de() {
    let example_raw = fs::read_to_string("./tests/mandos-io/example_raw.scen.json").unwrap();
    let example_normalized =
        fs::read_to_string("./tests/mandos-io/example_normalized.scen.json").unwrap();
    let scenario_raw = ScenarioRaw::from_json_str(example_raw.as_str());
    let scenario = Scenario::interpret_from(scenario_raw, &InterpreterContext::default());

    let scenario_raw_re = scenario.into_raw();
    let serialized = scenario_raw_re.to_json_string();

    let mut file = File::create("tests/mandos-io/serialized_normalized_1.scen.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    assert_eq!(serialized, example_normalized);
}

#[test]
fn test_scenario_high_to_high_ser_de() {
    let example_normalized =
        fs::read_to_string("./tests/mandos-io/example_normalized.scen.json").unwrap();
    let scenario_raw = ScenarioRaw::from_json_str(example_normalized.as_str());
    let scenario = Scenario::interpret_from(scenario_raw, &InterpreterContext::default());

    let scenario_raw_re = scenario.into_raw();
    let serialized = scenario_raw_re.to_json_string();

    let mut file = File::create("tests/mandos-io/serialized_normalized_2.scen.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    assert_eq!(serialized, example_normalized);
}
