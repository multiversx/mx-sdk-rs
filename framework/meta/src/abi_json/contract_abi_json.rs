use super::*;
use multiversx_sc::abi::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractAbiJson {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_info: Option<BuildInfoAbiJson>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub docs: Vec<String>,

    pub name: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constructor: Option<ConstructorAbiJson>,

    #[serde(default)]
    pub endpoints: Vec<EndpointAbiJson>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub promises_callback_names: Vec<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<EventAbiJson>,

    #[serde(default)]
    pub esdt_attributes: Vec<EsdtAttributeJson>,

    #[serde(default)]
    pub has_callback: bool,

    #[serde(default)]
    pub types: BTreeMap<String, TypeDescriptionJson>,
}

impl From<&ContractAbi> for ContractAbiJson {
    fn from(abi: &ContractAbi) -> Self {
        ContractAbiJson {
            build_info: Some(BuildInfoAbiJson::from(&abi.build_info)),
            docs: abi.docs.iter().map(|d| d.to_string()).collect(),
            name: abi.name.to_string(),
            constructor: abi.constructors.first().map(ConstructorAbiJson::from),
            endpoints: abi.endpoints.iter().map(EndpointAbiJson::from).collect(),
            promises_callback_names: abi
                .promise_callbacks
                .iter()
                .map(|endpoint| endpoint.name.to_string())
                .collect(),
            events: abi.events.iter().map(EventAbiJson::from).collect(),
            has_callback: abi.has_callback,
            types: convert_type_descriptions_to_json(&abi.type_descriptions),
            esdt_attributes: abi
                .esdt_attributes
                .iter()
                .map(EsdtAttributeJson::from)
                .collect(),
        }
    }
}

pub fn convert_type_descriptions_to_json(
    type_descriptions: &TypeDescriptionContainerImpl,
) -> BTreeMap<String, TypeDescriptionJson> {
    let mut types = BTreeMap::new();
    for (type_name, type_description) in type_descriptions.0.iter() {
        if type_description.contents.is_specified() {
            types.insert(
                type_name.clone(),
                TypeDescriptionJson::from(type_description),
            );
        }
    }
    types
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

pub fn deserialize_abi_from_json(input: &str) -> Result<ContractAbiJson, String> {
    serde_json::from_str(input).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    const MINIMAL_ABI_JSON: &str = r#"{
        "name": "Minimal"
    }"#;

    #[test]
    fn decode_minimal_contract_abi() {
        super::deserialize_abi_from_json(MINIMAL_ABI_JSON).unwrap();
    }
}
