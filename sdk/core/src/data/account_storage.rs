use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountStorage {
    pub pairs: HashMap<String, String>,
}

// AccountResponse holds the account endpoint response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountStorageResponse {
    pub data: Option<AccountStorage>,
    pub error: String,
    pub code: String,
}
