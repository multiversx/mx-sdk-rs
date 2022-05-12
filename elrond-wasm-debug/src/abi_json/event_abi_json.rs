use alloc::vec::Vec;
use elrond_wasm::abi::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EventInputAbiJson {
    #[serde(rename = "name")]
    pub arg_name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    /// Bool that is only serialized when true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed: Option<bool>,
}

impl From<&EventInputAbi> for EventInputAbiJson {
    fn from(abi: &EventInputAbi) -> Self {
        EventInputAbiJson {
            arg_name: abi.arg_name.to_string(),
            type_name: abi.type_name.clone(),
            indexed: if abi.indexed { Some(true) } else { None },
        }
    }
}

/// Same as EventAbiJson but ignores the name
#[derive(Serialize, Deserialize)]
pub struct ConstructorEventAbiJson {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub docs: Vec<String>,
    pub inputs: Vec<EventInputAbiJson>,
}

impl From<&EventAbi> for ConstructorEventAbiJson {
    fn from(abi: &EventAbi) -> Self {
        ConstructorEventAbiJson {
            docs: abi.docs.iter().map(|d| d.to_string()).collect(),
            inputs: abi.inputs.iter().map(EventInputAbiJson::from).collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EventAbiJson {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub docs: Vec<String>,
    pub name: String,
    pub inputs: Vec<EventInputAbiJson>,
}

impl From<&EventAbi> for EventAbiJson {
    fn from(abi: &EventAbi) -> Self {
        EventAbiJson {
            docs: abi.docs.iter().map(|d| d.to_string()).collect(),
            name: abi.name.to_string(),
            inputs: abi.inputs.iter().map(EventInputAbiJson::from).collect(),
        }
    }
}
