use super::*;
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    ser::{SerializeMap, SerializeSeq, Serializer},
    Deserialize, Serialize,
};
use std::fmt;

pub enum CheckEsdtRaw {
    Unspecified,
    Star,
    Equal(Vec<CheckEsdtDataRaw>),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckEsdtDataRaw {
    pub token_identifier: ValueSubTree,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckEsdtValuesRaw::is_unspecified")]
    pub instances: CheckEsdtValuesRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub last_nonce: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub roles: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub frozen: CheckBytesValueRaw,
}

#[derive(Deserialize)]
pub enum CheckEsdtValuesRaw {
    Unspecified,
    Star,
    Equal(Vec<CheckEsdtValueRaw>),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckEsdtValueRaw {
    pub nonce: ValueSubTree,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub balance: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub creator: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub royalties: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub hash: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub uri: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub attributes: CheckBytesValueRaw,
}

impl CheckEsdtRaw {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckEsdtRaw::Star)
    }

    pub fn is_unspecified(&self) -> bool {
        matches!(self, CheckEsdtRaw::Unspecified)
    }
}

impl Default for CheckEsdtRaw {
    fn default() -> Self {
        CheckEsdtRaw::Unspecified
    }
}

impl Serialize for CheckEsdtRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckEsdtRaw::Unspecified => {
                // empty map, just in case
                // won't get serialized anyway
                let map = serializer.serialize_map(Some(0))?;
                map.end()
            },
            CheckEsdtRaw::Star => serializer.serialize_str("*"),
            CheckEsdtRaw::Equal(m) => m.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for CheckEsdtRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckEsdtRawVisitor)
    }
}
struct CheckEsdtRawVisitor;

impl<'de> Visitor<'de> for CheckEsdtRawVisitor {
    type Value = CheckEsdtRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized object JSON representation of esdt check")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == "*" {
            Ok(CheckEsdtRaw::Star)
        } else {
            Err(de::Error::custom("only '*' allowed as esdt string value"))
        }
    }

    fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        Ok(CheckEsdtRaw::Equal(Deserialize::deserialize(
            de::value::MapAccessDeserializer::new(map),
        )?))
    }
}

impl CheckEsdtValuesRaw {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckEsdtValuesRaw::Star)
    }

    pub fn is_unspecified(&self) -> bool {
        matches!(self, CheckEsdtValuesRaw::Unspecified)
    }
}

impl Default for CheckEsdtValuesRaw {
    fn default() -> Self {
        CheckEsdtValuesRaw::Unspecified
    }
}

impl Serialize for CheckEsdtValuesRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckEsdtValuesRaw::Unspecified => {
                let map = serializer.serialize_map(Some(0))?;
                map.end()
            },
            CheckEsdtValuesRaw::Star => serializer.serialize_str("*"),
            CheckEsdtValuesRaw::Equal(m) => {
                let mut map = serializer.serialize_seq(Some(m.len()))?;
                for v in m {
                    map.serialize_element(v)?;
                }
                map.end()
            },
        }
    }
}
