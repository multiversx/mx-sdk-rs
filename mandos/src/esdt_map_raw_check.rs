use super::*;
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    ser::{SerializeMap, Serializer},
    Deserialize, Serialize,
};
use std::{collections::BTreeMap, fmt};
pub enum CheckEsdtMapRaw {
    Unspecified,
    Star,
    Equal(CheckEsdtMapContentsRaw),
}

pub struct CheckEsdtMapContentsRaw {
    pub contents: BTreeMap<String, CheckEsdtRaw>,
    pub other_esdts_allowed: bool,
}

impl CheckEsdtMapRaw {
    pub fn is_unspecified(&self) -> bool {
        matches!(self, CheckEsdtMapRaw::Unspecified)
    }

    pub fn is_star(&self) -> bool {
        matches!(self, CheckEsdtMapRaw::Star)
    }
}

impl Default for CheckEsdtMapRaw {
    fn default() -> Self {
        CheckEsdtMapRaw::Unspecified
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

impl Serialize for CheckEsdtMapContentsRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.contents.len()))?;
        for (k, v) in self.contents.iter() {
            map.serialize_entry(k, v)?;
        }
        if self.other_esdts_allowed {
            map.serialize_entry("+", "")?;
        }
        map.end()
    }
}
impl<'de> Deserialize<'de> for CheckEsdtMapContentsRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckEsdtMapContentsRawVisitor)
    }
}

struct CheckEsdtMapContentsRawVisitor;

impl<'de> Visitor<'de> for CheckEsdtMapContentsRawVisitor {
    type Value = CheckEsdtMapContentsRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("CheckAccountRaw or nothing")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut contents = BTreeMap::<String, CheckEsdtRaw>::new();

        // While there are entries remaining in the input, add them
        // into our map.
        let mut other_esdts_allowed = false;

        while let Some((key, value)) = access.next_entry()? {
            if key == "+" {
                other_esdts_allowed = true;
            } else {
                contents.insert(key, value);
            }
        }

        Ok(CheckEsdtMapContentsRaw {
            other_esdts_allowed,
            contents,
        })
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
