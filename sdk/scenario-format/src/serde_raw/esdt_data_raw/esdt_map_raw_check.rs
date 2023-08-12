use super::*;
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};
use std::fmt;

#[derive(Default)]
pub enum CheckEsdtMapRaw {
    #[default]
    Unspecified,
    Star,
    Equal(CheckEsdtMapContentsRaw),
}

impl CheckEsdtMapRaw {
    pub fn is_unspecified(&self) -> bool {
        matches!(self, CheckEsdtMapRaw::Unspecified)
    }

    pub fn is_star(&self) -> bool {
        matches!(self, CheckEsdtMapRaw::Star)
    }
}

impl Serialize for CheckEsdtMapRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckEsdtMapRaw::Unspecified => serializer.serialize_str(""),
            CheckEsdtMapRaw::Star => serializer.serialize_str("*"),
            CheckEsdtMapRaw::Equal(m) => m.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for CheckEsdtMapRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckEsdtMapRawVisitor)
    }
}
struct CheckEsdtMapRawVisitor;

impl<'de> Visitor<'de> for CheckEsdtMapRawVisitor {
    type Value = CheckEsdtMapRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized object JSON representation of log check")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == "*" {
            Ok(CheckEsdtMapRaw::Star)
        } else {
            Err(de::Error::custom("only '*' allowed as logs string value"))
        }
    }

    fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        Ok(CheckEsdtMapRaw::Equal(Deserialize::deserialize(
            de::value::MapAccessDeserializer::new(map),
        )?))
    }
}
