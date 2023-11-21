use std::collections::BTreeMap;

use multiversx_sc::abi::EsdtAttributeAbi;
use serde::{Deserialize, Serialize};

use super::{convert_type_descriptions_to_json, EsdtAttributeJson, TypeDescriptionJson};

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
