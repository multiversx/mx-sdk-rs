use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum LogData {
    #[default]
    Empty,
    String(String),
    Vec(Vec<String>),
}

impl LogData {
    pub fn for_each<F: FnMut(&str)>(&self, mut f: F) {
        match self {
            LogData::Empty => {}
            LogData::String(s) => f(s.as_str()),
            LogData::Vec(v) => v.iter().for_each(|s| f(s.as_str())),
        }
    }
}
