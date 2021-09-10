use super::*;
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    ser::{SerializeMap, Serializer},
    Deserialize, Serialize,
};
use std::{collections::BTreeMap, fmt};
pub enum CheckStorageRaw {
    Star,
    Equal(CheckStorageDetailsRaw),
}

pub struct CheckStorageDetailsRaw {
    pub storages: BTreeMap<String, CheckBytesValueRaw>,
    pub other_storages_allowed: bool,
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

impl Serialize for CheckStorageDetailsRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.storages.len()))?;
        for (k, v) in self.storages.iter() {
            map.serialize_entry(k, v)?;
        }
        if self.other_storages_allowed {
            map.serialize_entry("+", "")?;
        }
        map.end()
    }
}
impl<'de> Deserialize<'de> for CheckStorageDetailsRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckStorageDetailsRawVisitor)
    }
}

struct CheckStorageDetailsRawVisitor;

impl<'de> Visitor<'de> for CheckStorageDetailsRawVisitor {
    type Value = CheckStorageDetailsRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("CheckAccountRaw or nothing")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut storages = BTreeMap::<String, CheckBytesValueRaw>::new();

        // While there are entries remaining in the input, add them
        // into our map.
        let mut other_storages_allowed = false;

        while let Some((key, value)) = access.next_entry()? {
            if key == "+" {
                other_storages_allowed = true;
            } else {
                storages.insert(key, value);
            }
        }

        Ok(CheckStorageDetailsRaw {
            other_storages_allowed,
            storages,
        })
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

impl<'de> Deserialize<'de> for CheckStorageRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckStorageRawVisitor)
    }
}
