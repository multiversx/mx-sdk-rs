use super::CheckBytesValueRaw;
use serde::{
    de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, Serializer},
};
use std::fmt;

#[derive(Default)]
pub enum CheckValueListRaw {
    #[default]
    Unspecified,
    Star,
    CheckList(Vec<CheckBytesValueRaw>),
}

impl CheckValueListRaw {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckValueListRaw::Star)
    }

    pub fn is_unspecified(&self) -> bool {
        matches!(self, CheckValueListRaw::Unspecified)
    }
}

impl Serialize for CheckValueListRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckValueListRaw::Unspecified => serializer.serialize_str(""),
            CheckValueListRaw::Star => serializer.serialize_str("*"),
            CheckValueListRaw::CheckList(bytes_value) => bytes_value.serialize(serializer),
        }
    }
}

struct CheckValueListRawVisitor;

impl<'de> Visitor<'de> for CheckValueListRawVisitor {
    type Value = CheckValueListRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized CheckValueListRaw")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == "*" {
            Ok(CheckValueListRaw::Star)
        } else {
            Err(de::Error::custom(
                "only '*' allowed as check list string value",
            ))
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut list = Vec::<CheckBytesValueRaw>::new();

        while let Some(item) = seq.next_element()? {
            list.push(item);
        }

        Ok(CheckValueListRaw::CheckList(list))
    }
}

impl<'de> Deserialize<'de> for CheckValueListRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckValueListRawVisitor)
    }
}

impl fmt::Display for CheckValueListRaw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CheckValueListRaw::Unspecified => write!(f, ""),
            CheckValueListRaw::Star => write!(f, "*"),
            CheckValueListRaw::CheckList(check_values) => {
                write!(f, "[")?;
                for check_value in check_values {
                    write!(f, "{check_value}")?;
                }
                write!(f, "]")
            },
        }
    }
}
