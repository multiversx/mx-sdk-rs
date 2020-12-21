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
	pub types: BTreeMap<String, TypeDescriptionJson>,
}

impl From<&ContractAbi> for ContractAbiJson {
	fn from(abi: &ContractAbi) -> Self {
		let mut contract_json = ContractAbiJson {
			docs: abi.docs.iter().map(|d| d.to_string()).collect(),
			name: abi.name.to_string(),
			endpoints: Vec::new(),
			types: BTreeMap::new(),
		};
		for endpoint in &abi.endpoints {
			contract_json
				.endpoints
				.push(EndpointAbiJson::from(endpoint));
		}
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
