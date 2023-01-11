use multiversx_sc::abi::*;
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

#[derive(Serialize, Deserialize)]
pub struct EventAbiJson {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub docs: Vec<String>,
    pub identifier: String,
    pub inputs: Vec<EventInputAbiJson>,
}

impl From<&EventAbi> for EventAbiJson {
    fn from(abi: &EventAbi) -> Self {
        EventAbiJson {
            docs: abi.docs.iter().map(|d| d.to_string()).collect(),
            identifier: abi.identifier.to_string(),
            inputs: abi.inputs.iter().map(EventInputAbiJson::from).collect(),
        }
    }
}
