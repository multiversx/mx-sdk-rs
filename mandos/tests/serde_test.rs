extern crate mandos;

use std::{fs, fs::File, io::Write};

use mandos::serde_raw::{ScenarioRaw, StepRaw};
use serde::Serialize;

#[test]
fn test_scenario_raw_ser_de() {
    let contents = fs::read_to_string("./example.scen.json").unwrap();

    let scen: ScenarioRaw = serde_json::from_str(contents.as_str()).unwrap();

    // let serialized = serde_json::to_string_pretty(&scen).unwrap();
    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    scen.serialize(&mut ser).unwrap();
    let mut serialized = String::from_utf8(ser.into_inner()).unwrap();
    serialized.push('\n');

    let mut file = File::create("serialized.scen.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    assert_eq!(serialized, contents);
}
