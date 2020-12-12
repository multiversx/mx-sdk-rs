use alloc::vec::Vec;
use elrond_wasm::abi::*;

use super::*;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractAbiJson {
	pub docs: Vec<String>,
	pub endpoints: Vec<EndpointAbiJson>,
}

impl From<&ContractAbi> for ContractAbiJson {
	fn from(abi: &ContractAbi) -> Self {
		ContractAbiJson {
			docs: abi.docs.iter().map(|d| d.to_string()).collect(),
			endpoints: abi
				.endpoints
				.iter()
				.map(|e| EndpointAbiJson::from(e))
				.collect(),
		}
	}
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointAbiJson {
	pub docs: Vec<String>,
	pub name: String,
	pub payable: bool,
}

impl From<&EndpointAbi> for EndpointAbiJson {
	fn from(abi: &EndpointAbi) -> Self {
		EndpointAbiJson {
			docs: abi.docs.iter().map(|d| d.to_string()).collect(),
			name: abi.name.to_string(),
			payable: abi.payable,
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
