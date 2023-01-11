use super::address::Address;
use serde::{Deserialize, Serialize};

// Account holds an Account's information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub address: Address,
    pub nonce: u64,
    pub balance: String,
    pub code: String,
    pub code_hash: Option<Vec<u8>>,
    pub root_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountData {
    pub account: Account,
}

// AccountResponse holds the account endpoint response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountResponse {
    pub data: Option<AccountData>,
    pub error: String,
    pub code: String,
}
