use serde::{Deserialize, Serialize};

use crate::serde_raw::{TxESDTRaw, ValueSubTree};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxTransferRaw {
    pub from: ValueSubTree,
    pub to: ValueSubTree,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egld: Option<ValueSubTree>,

    #[serde(default)]
    pub esdt: Vec<TxESDTRaw>,
}
