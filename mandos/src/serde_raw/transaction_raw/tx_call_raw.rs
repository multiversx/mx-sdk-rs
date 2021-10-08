use crate::serde_raw::ValueSubTree;

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxCallRaw {
    pub from: ValueSubTree,
    pub to: ValueSubTree,
    pub value: ValueSubTree,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esdt: Option<TxESDTRaw>,

    pub function: String,

    #[serde(default)]
    pub arguments: Vec<ValueSubTree>,

    pub gas_limit: ValueSubTree,
    pub gas_price: ValueSubTree,
}
