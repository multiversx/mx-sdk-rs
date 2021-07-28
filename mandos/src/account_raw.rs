use super::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountRaw {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    pub nonce: ValueSubTree,
    pub balance: ValueSubTree,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esdt: Option<BTreeMap<String, ValueSubTree>>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<ValueSubTree>,

    #[serde(default)]
    pub storage: BTreeMap<String, ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<ValueSubTree>,
}
