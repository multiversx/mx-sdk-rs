use super::*;
use crate::serde_raw::ValueSubTree;
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};
use std::fmt;

pub enum CheckEsdtRaw {
    Short(ValueSubTree),
    Full(CheckEsdtDataRaw),
}

impl Serialize for CheckEsdtRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckEsdtRaw::Short(m) => m.serialize(serializer),
            CheckEsdtRaw::Full(m) => m.serialize(serializer),
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
        Ok(CheckEsdtRaw::Short(ValueSubTree::Str(value.to_string())))
    }

    fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        Ok(CheckEsdtRaw::Full(Deserialize::deserialize(
            de::value::MapAccessDeserializer::new(map),
        )?))
    }
}
