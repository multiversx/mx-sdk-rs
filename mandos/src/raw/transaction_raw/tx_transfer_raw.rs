use serde::{Deserialize, Serialize};

use crate::{TxESDTRaw, ValueSubTree};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxTransferRaw {
    pub from: ValueSubTree,
    pub to: ValueSubTree,
    pub value: ValueSubTree,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esdt: Option<TxESDTRaw>,
}
