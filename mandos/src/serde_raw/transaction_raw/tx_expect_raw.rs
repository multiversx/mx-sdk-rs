use crate::serde_raw::{CheckBytesValueRaw, CheckLogsRaw};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxExpectRaw {
    #[serde(default)]
    pub out: Vec<CheckBytesValueRaw>,

    #[serde(default)]
    pub status: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckLogsRaw::is_default")]
    pub logs: CheckLogsRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub message: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub gas: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub refund: CheckBytesValueRaw,
}
