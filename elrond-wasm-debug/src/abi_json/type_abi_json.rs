use alloc::vec::Vec;
use elrond_wasm::abi::*;

use super::*;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct TypeDescriptionJson {
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub docs: Vec<String>,
	pub name: String,

	#[serde(rename = "type")]
	pub content_type: String,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	#[serde(rename = "variants")]
	pub enum_variants: Vec<EnumVariantDescriptionJson>,
}

impl From<&TypeDescription> for TypeDescriptionJson {
	fn from(abi: &TypeDescription) -> Self {
		let mut type_desc_json = TypeDescriptionJson {
			docs: abi.docs.iter().map(|d| d.to_string()).collect(),
			name: abi.name.to_string(),
			content_type: match &abi.contents {
				TypeContents::NotSpecified => "not_specified",
				TypeContents::Enum(_) => "enum",
				TypeContents::Struct => "struct",
			}
			.to_string(),
			enum_variants: Vec::new(),
		};
		match &abi.contents {
			TypeContents::Enum(variants) => {
				for variant in variants {
					type_desc_json
						.enum_variants
						.push(EnumVariantDescriptionJson::from(variant));
				}
			},
			_ => {},
		}

		type_desc_json
	}
}

#[derive(Serialize, Deserialize)]
pub struct EnumVariantDescriptionJson {
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub docs: Vec<String>,
	pub name: String,
}

impl From<&EnumVariantDescription> for EnumVariantDescriptionJson {
	fn from(abi: &EnumVariantDescription) -> Self {
		EnumVariantDescriptionJson {
			docs: abi.docs.iter().map(|d| d.to_string()).collect(),
			name: abi.name.to_string(),
		}
	}
}
