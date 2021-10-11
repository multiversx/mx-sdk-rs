use serde::{Deserialize, Serialize};

use crate::serde_raw::{TxESDTRaw, ValueSubTree};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxTransferRaw {
    pub from: ValueSubTree,
    pub to: ValueSubTree,
    pub value: Option<ValueSubTree>,
    pub egld: Option<ValueSubTree>,
    pub esdt: Vec<TxESDTRaw>,
}
