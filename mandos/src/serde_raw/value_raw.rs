use serde::{
    de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor},
    ser::{Serialize, SerializeMap, SerializeSeq, Serializer},
};
use std::{collections::BTreeMap, fmt};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ValueSubTree {
    Str(String),
    List(Vec<ValueSubTree>),
    Map(BTreeMap<String, ValueSubTree>),
}

impl ValueSubTree {
    pub fn is_empty_string(&self) -> bool {
        match self {
            ValueSubTree::Str(s) => s.is_empty(),
            _ => false,
        }
    }

    fn append_to_concatenated_string(&self, accumulator: &mut String) {
        match self {
            ValueSubTree::Str(s) => accumulator.push_str(s.as_str()),
            ValueSubTree::List(l) => {
                for item in l {
                    if !(accumulator.is_empty()) {
                        accumulator.push('|');
                    }
                    item.append_to_concatenated_string(accumulator);
                }
            },
            ValueSubTree::Map(m) => {
                for value in m.values() {
                    if !(accumulator.is_empty()) {
                        accumulator.push('|');
                    }
                    value.append_to_concatenated_string(accumulator);
                }
            },
        }
    }

    /// Concatenates all contained values into a String.
    pub fn to_concatenated_string(&self) -> String {
        let mut accumulator = String::new();
        self.append_to_concatenated_string(&mut accumulator);
        accumulator
    }
}

impl Default for ValueSubTree {
    fn default() -> Self {
        ValueSubTree::Str(String::from(""))
    }
}

impl Serialize for ValueSubTree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ValueSubTree::Str(s) => serializer.serialize_str(s.as_str()),
            ValueSubTree::List(l) => {
                let mut seq = serializer.serialize_seq(Some(l.len()))?;
                for item in l {
                    seq.serialize_element(item)?;
                }
                seq.end()
            },
            ValueSubTree::Map(m) => {
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            },
        }
    }
}

pub(crate) struct ValueSubTreeVisitor;

impl<'de> Visitor<'de> for ValueSubTreeVisitor {
    type Value = ValueSubTree;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized object JSON representation")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ValueSubTree::Str(String::from(value)))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut list = Vec::<ValueSubTree>::new();

        while let Some(item) = seq.next_element()? {
            list.push(item);
        }

        Ok(ValueSubTree::List(list))
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

        Ok(ValueSubTree::Map(map))
    }
}

impl<'de> Deserialize<'de> for ValueSubTree {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueSubTreeVisitor)
    }
}

impl fmt::Display for ValueSubTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
