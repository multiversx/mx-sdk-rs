use super::*;
use std::fmt;

use serde::{
    de::{self, Deserializer, SeqAccess, Visitor},
    ser::{SerializeSeq, Serializer},
    Deserialize, Serialize,
};

#[derive(Default)]
pub struct CheckLogListRaw {
    pub list: Vec<CheckLogRaw>,
    pub more_allowed_at_end: bool,
}

#[derive(Default)]
pub enum CheckLogsRaw {
    Star,
    List(CheckLogListRaw),
    #[default]
    Unspecified,
}

impl CheckLogsRaw {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckLogsRaw::Star)
    }

    pub fn is_default(&self) -> bool {
        matches!(self, CheckLogsRaw::Unspecified)
    }
}

impl Serialize for CheckLogsRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckLogsRaw::Unspecified => serializer.serialize_str(""),
            CheckLogsRaw::Star => serializer.serialize_str("*"),
            CheckLogsRaw::List(l) => {
                let mut seq = serializer.serialize_seq(Some(l.list.len()))?;
                for item in &l.list {
                    seq.serialize_element(item)?;
                }
                if l.more_allowed_at_end {
                    seq.serialize_element("+")?;
                }
                seq.end()
            },
        }
    }
}

/// Temporary value, just for loading the check log list.
enum CheckLogElement {
    Log(CheckLogRaw),
    Plus,
}

struct CheckLogElementVisitor;

impl<'de> Visitor<'de> for CheckLogElementVisitor {
    type Value = CheckLogElement;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized object JSON representation of log check")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == "+" {
            Ok(CheckLogElement::Plus)
        } else {
            Err(de::Error::custom(
                "only '+' allowed as log entry string value",
            ))
        }
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let log = CheckLogRaw::deserialize(de::value::MapAccessDeserializer::new(map))?;
        Ok(CheckLogElement::Log(log))
    }
}

impl<'de> Deserialize<'de> for CheckLogElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckLogElementVisitor)
    }
}

struct CheckLogsVisitor;

impl<'de> Visitor<'de> for CheckLogsVisitor {
    type Value = CheckLogsRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized object JSON representation of log check")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == "*" {
            Ok(CheckLogsRaw::Star)
        } else {
            Err(de::Error::custom("only '*' allowed as logs string value"))
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut check_list = CheckLogListRaw::default();

        while let Some(entry) = seq.next_element::<CheckLogElement>()? {
            match entry {
                CheckLogElement::Log(log) => {
                    if check_list.more_allowed_at_end {
                        return Err(de::Error::custom(
                            "in check log list \"+\" can only be placed last",
                        ));
                    }
                    check_list.list.push(log);
                },
                CheckLogElement::Plus => {
                    check_list.more_allowed_at_end = true;
                },
            }
        }

        Ok(CheckLogsRaw::List(check_list))
    }
}

impl<'de> Deserialize<'de> for CheckLogsRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckLogsVisitor)
    }
}
