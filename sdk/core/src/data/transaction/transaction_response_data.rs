use serde::{Deserialize, Serialize};

/// Corresponds to [`TransactionResponseData`](https://github.com/multiversx/mx-chain-proxy-go/blob/master/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResponseData {
    pub tx_hash: String,
}

/// Corresponds to [`ResponseTransaction`](https://github.com/multiversx/mx-chain-proxy-go/blob/master/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTransaction {
    pub error: String,
    pub code: String,
    pub data: Option<TransactionResponseData>,
}
