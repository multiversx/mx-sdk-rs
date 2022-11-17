use super::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CheckLogRaw {
    pub address: CheckBytesValueRaw,

    pub endpoint: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckValueListRaw::is_unspecified")]
    pub topics: CheckValueListRaw,

    pub data: CheckBytesValueRaw,
}
