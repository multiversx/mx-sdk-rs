use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransactionsResponseData {
    pub num_of_sent_txs: i32,
    pub txs_hashes: HashMap<i32, String>,
}

// SendTransactionsResponse holds the response received from the network when broadcasting multiple transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionsResponse {
    pub error: String,
    pub code: String,
    pub data: Option<SendTransactionsResponseData>,
}
