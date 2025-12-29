use std::collections::BTreeMap;

use multiversx_sc::abi::EsdtAttributeAbi;
use serde::{Deserialize, Serialize};

use super::{
    EsdtAttributeJson, TypeDescriptionJson, convert_type_descriptions_to_json,
    empty_type_description_container,
};

/// Represents an entire ESDT attribute ABI file. The type descriptions only show up here.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EsdtAttributeAbiJson {
    pub esdt_attribute: EsdtAttributeJson,

    #[serde(default)]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub types: BTreeMap<String, TypeDescriptionJson>,
}

impl EsdtAttributeAbiJson {
    pub fn new(attr: &EsdtAttributeAbi) -> Self {
        EsdtAttributeAbiJson {
            esdt_attribute: EsdtAttributeJson::from(attr),
            types: convert_type_descriptions_to_json(&attr.type_descriptions),
        }
    }
}

impl From<&EsdtAttributeAbiJson> for EsdtAttributeAbi {
    fn from(abi_json: &EsdtAttributeAbiJson) -> Self {
        EsdtAttributeAbi {
            ticker: abi_json.esdt_attribute.ticker.clone(),
            ty: abi_json.esdt_attribute.ty.clone(),
            type_descriptions: empty_type_description_container(), // TODO: @Laur should recursively call convert_json_to_type_descriptions
        }
    }
}
