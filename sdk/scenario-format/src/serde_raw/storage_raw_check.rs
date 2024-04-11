use super::*;
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};
use std::{collections::BTreeMap, fmt};
pub enum CheckStorageRaw {
    Star,
    Equal(CheckStorageDetailsRaw),
}

impl CheckStorageRaw {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckStorageRaw::Star)
    }
}

impl Default for CheckStorageRaw {
    fn default() -> Self {
        CheckStorageRaw::Equal(CheckStorageDetailsRaw {
            storages: BTreeMap::new(),
            other_storages_allowed: false,
        })
    }
}

impl Serialize for CheckStorageRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckStorageRaw::Star => serializer.serialize_str("*"),
            CheckStorageRaw::Equal(m) => m.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for CheckStorageRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckStorageRawVisitor)
    }
}

struct CheckStorageRawVisitor;

impl<'de> Visitor<'de> for CheckStorageRawVisitor {
    type Value = CheckStorageRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized object JSON representation of log check")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == "*" {
            Ok(CheckStorageRaw::Star)
        } else {
            Err(de::Error::custom("only '*' allowed as logs string value"))
        }
    }

    fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        Ok(CheckStorageRaw::Equal(Deserialize::deserialize(
            de::value::MapAccessDeserializer::new(map),
        )?))
    }
}
