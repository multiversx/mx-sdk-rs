use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::address::Address;

// Account holds an Account's information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EsdtBalance {
    pub token_identifier: String,
    pub balance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royalties: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uris: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EsdtBalanceData {
    pub esdts: HashMap<String, EsdtBalance>,
}

// EsdtBalanceResponse holds the esdt balance endpoint response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EsdtBalanceResponse {
    pub data: Option<EsdtBalanceData>,
    pub error: String,
    pub code: String,
}
