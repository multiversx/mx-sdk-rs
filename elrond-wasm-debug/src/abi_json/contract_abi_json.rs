use super::*;
use alloc::vec::Vec;
use elrond_wasm::abi::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractAbiJson {
    pub build_info: BuildInfoAbiJson,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub docs: Vec<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constructor: Option<ConstructorAbiJson>,
    pub endpoints: Vec<EndpointAbiJson>,
    pub has_callback: bool,
    pub types: BTreeMap<String, TypeDescriptionJson>,
}

impl From<&ContractAbi> for ContractAbiJson {
    fn from(abi: &ContractAbi) -> Self {
        let mut contract_json = ContractAbiJson {
            build_info: BuildInfoAbiJson::from(&abi.build_info),
            docs: abi.docs.iter().map(|d| d.to_string()).collect(),
            name: abi.name.to_string(),
            constructor: abi.constructors.get(0).map(ConstructorAbiJson::from),
            endpoints: abi.endpoints.iter().map(EndpointAbiJson::from).collect(),
            has_callback: abi.has_callback,
            types: BTreeMap::new(),
        };
        for (type_name, type_description) in abi.type_descriptions.0.iter() {
            if type_description.contents.is_specified() {
                contract_json.types.insert(
                    type_name.clone(),
                    TypeDescriptionJson::from(type_description),
                );
            }
        }
        contract_json
    }
}

pub fn serialize_abi_to_json(abi_json: &ContractAbiJson) -> String {
    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    abi_json.serialize(&mut ser).unwrap();
    let mut serialized = String::from_utf8(ser.into_inner()).unwrap();
    serialized.push('\n');
    serialized
}
