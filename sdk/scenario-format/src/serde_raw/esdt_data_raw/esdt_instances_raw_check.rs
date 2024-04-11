use super::*;
use serde::{
    de::{self, Deserializer, SeqAccess, Visitor},
    ser::{SerializeMap, SerializeSeq, Serializer},
    Deserialize, Serialize,
};
use std::fmt;

#[derive(Default)]
pub enum CheckEsdtInstancesRaw {
    #[default]
    Unspecified,
    Star,
    Equal(Vec<CheckEsdtInstanceRaw>),
}

impl CheckEsdtInstancesRaw {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckEsdtInstancesRaw::Star)
    }

    pub fn is_unspecified(&self) -> bool {
        matches!(self, CheckEsdtInstancesRaw::Unspecified)
    }
}

impl Serialize for CheckEsdtInstancesRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckEsdtInstancesRaw::Unspecified => {
                let map = serializer.serialize_map(Some(0))?;
                map.end()
            },
            CheckEsdtInstancesRaw::Star => serializer.serialize_str("*"),
            CheckEsdtInstancesRaw::Equal(m) => {
                let mut map = serializer.serialize_seq(Some(m.len()))?;
                for v in m {
                    map.serialize_element(v)?;
                }
                map.end()
            },
        }
    }
}

struct CheckEsdtInstancesRawVisitor;

impl<'de> Visitor<'de> for CheckEsdtInstancesRawVisitor {
    type Value = CheckEsdtInstancesRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized object JSON representation of an ESDT instances list")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == "*" {
            Ok(CheckEsdtInstancesRaw::Star)
        } else {
            Err(de::Error::custom(
                "only '*' allowed as ESDT instances value",
            ))
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut list = Vec::<CheckEsdtInstanceRaw>::new();

        while let Some(item) = seq.next_element()? {
            list.push(item);
        }

        Ok(CheckEsdtInstancesRaw::Equal(list))
    }
}

impl<'de> Deserialize<'de> for CheckEsdtInstancesRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckEsdtInstancesRawVisitor)
    }
}
