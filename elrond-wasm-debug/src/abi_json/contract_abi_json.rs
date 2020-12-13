use alloc::vec::Vec;
use elrond_wasm::abi::*;

use super::*;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct ContractAbiJson {
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub docs: Vec<String>,
	pub name: String,
	pub endpoints: Vec<EndpointAbiJson>,
}

impl From<&ContractAbi> for ContractAbiJson {
	fn from(abi: &ContractAbi) -> Self {
		ContractAbiJson {
			docs: abi.docs.iter().map(|d| d.to_string()).collect(),
			name: abi.name.to_string(),
			endpoints: abi
				.endpoints
				.iter()
				.map(|e| EndpointAbiJson::from(e))
				.collect(),
		}
	}
}

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
			type_name: abi.type_name.clone(),
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
			type_name: abi.type_name.clone(),
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

pub fn serialize_abi_to_json(abi: &ContractAbi) -> String {
	let abi_json = ContractAbiJson::from(abi);
	let buf = Vec::new();
	let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
	let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
	abi_json.serialize(&mut ser).unwrap();
	let mut serialized = String::from_utf8(ser.into_inner()).unwrap();
	serialized.push('\n');
	serialized
}
