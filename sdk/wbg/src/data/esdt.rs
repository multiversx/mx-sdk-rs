use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// EsdtBalance  holds information about the esdt balance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EsdtBalance {
    pub token_identifier: String,
    pub balance: String,
}

// EsdtBalanceDataholds the esdt balance data
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

// EsdtRolesData holds the esdt roles data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EsdtRolesData {
    pub roles: HashMap<String, Vec<String>>,
}

// EsdtBalanceResponse holds the esdt roles endpoint response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EsdtRolesResponse {
    pub data: Option<EsdtRolesData>,
    pub error: String,
    pub code: String,
}
