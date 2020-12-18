use alloc::vec::Vec;
use elrond_wasm::abi::*;

use super::*;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct InputAbiJson {
	#[serde(rename = "name")]
	pub arg_name: String,
	#[serde(rename = "type")]
	pub type_name: String,
}

impl From<&InputAbi> for InputAbiJson {
	fn from(abi: &InputAbi) -> Self {
		InputAbiJson {
			arg_name: abi.arg_name.to_string(),
			type_name: abi.type_description.name.clone(),
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct OutputAbiJson {
	#[serde(rename = "type")]
	pub type_name: String,
}

impl From<&OutputAbi> for OutputAbiJson {
	fn from(abi: &OutputAbi) -> Self {
		OutputAbiJson {
			type_name: abi.type_description.name.clone(),
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct EndpointAbiJson {
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub docs: Vec<String>,
	pub name: String,
	pub payable: bool,
	pub inputs: Vec<InputAbiJson>,
	pub outputs: Vec<OutputAbiJson>,
}

impl From<&EndpointAbi> for EndpointAbiJson {
	fn from(abi: &EndpointAbi) -> Self {
		EndpointAbiJson {
			docs: abi.docs.iter().map(|d| d.to_string()).collect(),
			name: abi.name.to_string(),
			payable: abi.payable,
			inputs: abi
				.inputs
				.iter()
				.map(|input| InputAbiJson::from(input))
				.collect(),
			outputs: abi
				.outputs
				.iter()
				.map(|output| OutputAbiJson::from(output))
				.collect(),
		}
	}
}
