use std::{collections::BTreeMap, fmt};

use super::*;
use serde::{
    de::{Deserializer, MapAccess, Visitor},
    ser::{SerializeMap, Serializer},
    Deserialize, Serialize,
};
pub struct CheckStorageDetailsRaw {
    pub storages: BTreeMap<String, CheckBytesValueRaw>,
    pub other_storages_allowed: bool,
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
