use super::*;
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};
use std::fmt;

pub enum EsdtRaw {
    Short(String),
    Full(EsdtFullRaw),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EsdtFullRaw {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_identifier: Option<ValueSubTree>,

    #[serde(default)]
    pub instances: Vec<InstanceRaw>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_nonce: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen: Option<ValueSubTree>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceRaw {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royalties: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<ValueSubTree>,
}

impl Serialize for EsdtRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            EsdtRaw::Short(m) => m.serialize(serializer),
            EsdtRaw::Full(m) => m.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for EsdtRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckEsdtRawVisitor)
    }
}
struct CheckEsdtRawVisitor;

impl<'de> Visitor<'de> for CheckEsdtRawVisitor {
    type Value = EsdtRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized object JSON representation of esdt check")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(EsdtRaw::Short(value.to_string()))
    }

    fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        Ok(EsdtRaw::Full(Deserialize::deserialize(
            de::value::MapAccessDeserializer::new(map),
        )?))
    }
}
