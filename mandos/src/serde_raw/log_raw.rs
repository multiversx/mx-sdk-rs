use super::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CheckLogRaw {
    pub address: ValueSubTree,

    pub endpoint: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<CheckBytesValueRaw>,

    pub data: CheckBytesValueRaw,
}
