use serde::{Deserialize, Serialize};

/// Corresponds to [`ResponseTxStatus`](https://github.com/multiversx/mx-chain-proxy-go/blob/main/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTxStatus {
    pub status: String,
}

/// Response envelope for the `/transaction/{hash}/status` endpoint.
/// The proxy constructs the data field inline; there is no named wrapper type in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStatus {
    pub error: String,
    pub code: String,
    pub data: Option<ResponseTxStatus>,
}
