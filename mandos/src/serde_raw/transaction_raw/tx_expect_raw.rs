use crate::serde_raw::{CheckBytesValueRaw, CheckLogsRaw, CheckValueListRaw};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxExpectRaw {
    #[serde(default)]
    #[serde(skip_serializing_if = "CheckValueListRaw::is_unspecified")]
    pub out: CheckValueListRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub status: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub message: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckLogsRaw::is_default")]
    pub logs: CheckLogsRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub gas: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub refund: CheckBytesValueRaw,
}
