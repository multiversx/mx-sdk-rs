use serde::{Deserialize, Serialize};

use crate::serde_raw::{TxESDTRaw, ValueSubTree};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxTransferRaw {
    pub from: ValueSubTree,
    pub to: ValueSubTree,

    /// Backwards compatibility only.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egld_value: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub esdt_value: Vec<TxESDTRaw>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<ValueSubTree>,
}
