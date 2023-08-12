use super::value_raw::*;
use serde::{
    de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor},
    ser::{Serialize, Serializer},
};
use std::fmt;

#[derive(Default)]
pub enum CheckBytesValueRaw {
    #[default]
    Unspecified,
    Star,
    Equal(ValueSubTree),
}

impl CheckBytesValueRaw {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckBytesValueRaw::Star)
    }

    pub fn is_unspecified(&self) -> bool {
        matches!(self, CheckBytesValueRaw::Unspecified)
    }
}

impl Serialize for CheckBytesValueRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckBytesValueRaw::Unspecified => serializer.serialize_str(""),
            CheckBytesValueRaw::Star => serializer.serialize_str("*"),
            CheckBytesValueRaw::Equal(bytes_value) => bytes_value.serialize(serializer),
        }
    }
}

struct CheckBytesValueRawVisitor;

impl<'de> Visitor<'de> for CheckBytesValueRawVisitor {
    type Value = CheckBytesValueRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized CheckBytesValueRaw")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == "*" {
            Ok(CheckBytesValueRaw::Star)
        } else {
            let vst = ValueSubTreeVisitor.visit_str(value)?;
            Ok(CheckBytesValueRaw::Equal(vst))
        }
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let vst = ValueSubTreeVisitor.visit_seq(seq)?;
        Ok(CheckBytesValueRaw::Equal(vst))
    }

    fn visit_map<M>(self, access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let vst = ValueSubTreeVisitor.visit_map(access)?;
        Ok(CheckBytesValueRaw::Equal(vst))
    }
}

impl<'de> Deserialize<'de> for CheckBytesValueRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckBytesValueRawVisitor)
    }
}

impl fmt::Display for CheckBytesValueRaw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CheckBytesValueRaw::Unspecified => write!(f, ""),
            CheckBytesValueRaw::Star => write!(f, "*"),
            CheckBytesValueRaw::Equal(bytes_value) => bytes_value.fmt(f),
        }
    }
}
