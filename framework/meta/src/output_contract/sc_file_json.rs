use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::Path};

use crate::abi_json::ContractAbiJson;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScFileJson {
    pub abi: ContractAbiJson,
    pub size: usize,
    pub code: String,
}

pub fn serialize_sc_file_json(sc_file_json: &ScFileJson) -> String {
    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    sc_file_json.serialize(&mut ser).unwrap();
    let mut serialized = String::from_utf8(ser.into_inner()).unwrap();
    serialized.push('\n');
    serialized
}

pub fn save_sc_file_json(sc_file_json: &ScFileJson, path: impl AsRef<Path>) {
    let sc_file_string = serialize_sc_file_json(sc_file_json);
    let mut sc_file = File::create(path).unwrap();
    write!(sc_file, "{sc_file_string}").unwrap();
}
