use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Account holds an Account's information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EsdtBalance {
    pub token_identifier: String,
    pub balance: String,
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
