use serde::{Deserialize, Serialize};

use crate::serde_raw::{CheckBytesValueRaw, CheckValueListRaw, ValueSubTree};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckEsdtInstanceRaw {
    pub nonce: ValueSubTree,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub balance: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub creator: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub royalties: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub hash: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckValueListRaw::is_unspecified")]
    pub uri: CheckValueListRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub attributes: CheckBytesValueRaw,
}
