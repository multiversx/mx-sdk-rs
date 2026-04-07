use serde::{Deserialize, Serialize};

/// Represents the format of the data field of a transaction response.
///
/// Corresponds to [`TransactionResponseData`](https://github.com/multiversx/mx-chain-proxy-go/blob/master/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResponseData {
    pub tx_hash: String,
}

/// Defines a response tx holding the resulting hash.
///
/// Corresponds to [`ResponseTransaction`](https://github.com/multiversx/mx-chain-proxy-go/blob/master/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTransaction {
    pub error: String,
    pub code: String,
    pub data: Option<TransactionResponseData>,
}
