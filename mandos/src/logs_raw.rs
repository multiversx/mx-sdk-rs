use super::*;
use std::fmt;

use serde::de::{self, Deserializer, SeqAccess, Visitor};
use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CheckLogRaw {
	pub address: ValueSubTree,
	pub identifier: ValueSubTree,

	#[serde(default)]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub topics: Vec<ValueSubTree>,

	pub data: ValueSubTree,
}

pub enum CheckLogsRaw {
	Star,
	List(Vec<CheckLogRaw>),
	Unspecified,
}

impl CheckLogsRaw {
	pub fn is_star(&self) -> bool {
		matches!(self, CheckLogsRaw::Star)
	}

	pub fn is_default(&self) -> bool {
		matches!(self, CheckLogsRaw::Unspecified)
	}
}

impl Default for CheckLogsRaw {
	fn default() -> Self {
		CheckLogsRaw::Unspecified
	}
}

impl Serialize for CheckLogsRaw {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match self {
			CheckLogsRaw::Unspecified => serializer.serialize_str(""),
			CheckLogsRaw::Star => serializer.serialize_str("*"),
			CheckLogsRaw::List(l) => {
				let mut seq = serializer.serialize_seq(Some(l.len()))?;
				for item in l {
					seq.serialize_element(item)?;
				}
				seq.end()
			},
		}
	}
}

struct CheckLogsVisitor;

impl<'de> Visitor<'de> for CheckLogsVisitor {
	type Value = CheckLogsRaw;

	// Format a message stating what data this Visitor expects to receive.
	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("serialized object JSON representation of log check")
	}

	fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		if value == "*" {
			Ok(CheckLogsRaw::Star)
		} else {
			Err(de::Error::custom("only '*' allowed as logs string value"))
		}
	}

	fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
	where
		A: SeqAccess<'de>,
	{
		let mut list = Vec::<CheckLogRaw>::new();

		while let Some(item) = seq.next_element()? {
			list.push(item);
		}

		Ok(CheckLogsRaw::List(list))
	}
}

impl<'de> Deserialize<'de> for CheckLogsRaw {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(CheckLogsVisitor)
	}
}
