use super::*;
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    ser::{SerializeMap, Serializer},
    Deserialize, Serialize,
};
use std::{collections::BTreeMap, fmt};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckAccountRaw {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub nonce: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub balance: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckEsdtRaw::is_unspecified")]
    pub esdt: CheckEsdtRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub username: CheckBytesValueRaw,

    #[serde(default)]
    pub storage: CheckStorageRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub code: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub async_call_data: CheckBytesValueRaw,
}

pub enum CheckAccountRawOrNothing {
    Some(CheckAccountRaw),
    Nothing,
}

struct CheckAccountRawOrNothingVisitor;

impl<'de> Visitor<'de> for CheckAccountRawOrNothingVisitor {
    type Value = CheckAccountRawOrNothing;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("CheckAccountRaw or nothing")
    }

    fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(CheckAccountRawOrNothing::Nothing)
    }

    fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        Ok(CheckAccountRawOrNothing::Some(Deserialize::deserialize(
            de::value::MapAccessDeserializer::new(map),
        )?))
    }
}

impl<'de> Deserialize<'de> for CheckAccountRawOrNothing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckAccountRawOrNothingVisitor)
    }
}

pub struct CheckAccountsRaw {
    pub other_accounts_allowed: bool,
    pub accounts: BTreeMap<String, CheckAccountRaw>,
}

impl Serialize for CheckAccountsRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.accounts.len()))?;
        for (k, v) in self.accounts.iter() {
            map.serialize_entry(k, v)?;
        }
        if self.other_accounts_allowed {
            map.serialize_entry("+", "")?;
        }
        map.end()
    }
}

struct CheckAccountRawsVisitor;

impl<'de> Visitor<'de> for CheckAccountRawsVisitor {
    type Value = CheckAccountsRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized CheckAccountsRaw")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut accounts = BTreeMap::<String, CheckAccountRaw>::new();
        let mut other_accounts_allowed = false;

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_entry()? {
            if key == "+" {
                other_accounts_allowed = true;
            } else if let CheckAccountRawOrNothing::Some(check_account) = value {
                accounts.insert(key, check_account);
            } else {
                return Err(de::Error::custom("invalid CheckAccountRaw"));
            }
        }

        Ok(CheckAccountsRaw {
            other_accounts_allowed,
            accounts,
        })
    }
}

impl<'de> Deserialize<'de> for CheckAccountsRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckAccountRawsVisitor)
    }
}
