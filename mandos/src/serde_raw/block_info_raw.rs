use serde::{Deserialize, Serialize};

use crate::serde_raw::ValueSubTree;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfoRaw {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_timestamp: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_nonce: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_round: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_epoch: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_random_seed: Option<ValueSubTree>,
}
