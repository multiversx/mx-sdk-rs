use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionsResponseData {
    #[serde(rename = "txsSent")]
    pub num_of_sent_txs: u64,
    #[serde(rename = "txsHashes")]
    pub txs_hashes: HashMap<u64, String>,
}

// SendTransactionsResponse holds the response received from the network when broadcasting multiple transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionsResponse {
    pub error: String,
    pub code: String,
    pub data: Option<SendTransactionsResponseData>,
}
