use serde::{Deserialize, Serialize};

use super::transaction_on_network::ApiTransactionResult;

/// Corresponds to [`GetTransactionResponseData`](https://github.com/multiversx/mx-chain-proxy-go/blob/main/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransactionResponseData {
    pub transaction: ApiTransactionResult,
}

/// Corresponds to [`GetTransactionResponse`](https://github.com/multiversx/mx-chain-proxy-go/blob/main/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransactionResponse {
    #[serde(default)]
    pub error: String,
    pub code: String,
    pub data: Option<GetTransactionResponseData>,
}
