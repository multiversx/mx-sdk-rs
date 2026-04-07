use serde::{Deserialize, Serialize};

/// Holds the process status of a transaction.
///
/// Corresponds to [`ProcessStatusResponse`](https://github.com/multiversx/mx-chain-proxy-go/blob/master/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessStatusResponse {
    pub reason: String,
    pub status: String,
}

/// Response envelope for the `/transaction/{hash}/process-status` endpoint.
/// The proxy constructs the data field inline; there is no named wrapper type in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionProcessStatus {
    pub error: String,
    pub code: String,
    pub data: Option<ProcessStatusResponse>,
}
