use serde::{Deserialize, Serialize};

use crate::ValueSubTree;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxESDTRaw {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_identifier: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<ValueSubTree>,

    pub value: ValueSubTree,
}
