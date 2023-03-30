use serde::{Deserialize, Serialize};

use crate::serde_raw::ValueSubTree;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EsdtInstanceRaw {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royalties: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub uri: Vec<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<ValueSubTree>,
}
