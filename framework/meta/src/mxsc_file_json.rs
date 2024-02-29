use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::Path};

use crate::abi_json::{BuildInfoAbiJson, ContractAbiJson};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MxscFileJson {
    pub build_info: BuildInfoAbiJson,
    pub abi: ContractAbiJson,
    pub size: usize,
    pub code: String,
}

pub fn serialize_mxsc_file_json(mxsc_file_json: &MxscFileJson) -> String {
    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    mxsc_file_json.serialize(&mut ser).unwrap();
    let mut serialized = String::from_utf8(ser.into_inner()).unwrap();
    serialized.push('\n');
    serialized
}

pub fn save_mxsc_file_json(mxsc_file_json: &MxscFileJson, path: impl AsRef<Path>) {
    let mxsc_file_string = serialize_mxsc_file_json(mxsc_file_json);
    let mut mxsc_file = File::create(path).unwrap();
    write!(mxsc_file, "{mxsc_file_string}").unwrap();
}
