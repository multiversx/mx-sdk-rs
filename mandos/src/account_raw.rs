use super::*;
use std::fmt;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use serde::ser::{Serializer, SerializeMap};
use serde::de::{self, Deserializer, Visitor, MapAccess};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountRaw {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    pub nonce: ValueSubTree,
    pub balance: ValueSubTree,
    pub storage: BTreeMap<String, ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esdt: Option<BTreeMap<String, ValueSubTree>>,
    
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ValueSubTree>,
}

pub enum CheckStorageRaw {
    Star,
    Equal(BTreeMap<String, ValueSubTree>)
}

impl CheckStorageRaw {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckStorageRaw::Star)
    }
}

impl Serialize for CheckStorageRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckStorageRaw::Star => serializer.serialize_str("*"),
            CheckStorageRaw::Equal(m) => {
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            },
        }
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

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = BTreeMap::<String, ValueSubTree>::new();

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(CheckStorageRaw::Equal(map))
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckAccountRaw {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "ValueSubTree::is_empty_string")]
    pub nonce: ValueSubTree,

    #[serde(default)]
    #[serde(skip_serializing_if = "ValueSubTree::is_empty_string")]
    pub balance: ValueSubTree,

    pub storage: CheckStorageRaw,
    
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "ValueSubTree::is_empty_string")]
    pub async_call_data: ValueSubTree,
}

pub enum CheckAccountRawOrNothing {
    Some(CheckAccountRaw),
    Nothing
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
        Ok(CheckAccountRawOrNothing::Some(Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?))
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
    pub accounts: BTreeMap<String, CheckAccountRaw>
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
                return Err(de::Error::custom("invalid CheckAccountRaw"))
            }
        }

        Ok(CheckAccountsRaw {
            accounts,
            other_accounts_allowed
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
