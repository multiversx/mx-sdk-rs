use super::*;
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    ser::{SerializeMap, Serializer},
    Deserialize, Serialize,
};
use std::{collections::BTreeMap, fmt};

pub enum CheckEsdtRaw {
    Unspecified,
    Star,
    Equal(BTreeMap<String, CheckBytesValueRaw>),
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
            CheckEsdtRaw::Equal(m) => {
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            },
        }
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

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = BTreeMap::<String, CheckBytesValueRaw>::new();

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(CheckEsdtRaw::Equal(map))
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
