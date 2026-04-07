use serde::{Deserialize, Serialize};

use super::api_transaction_result::ApiTransactionResult;

/// Corresponds to [`GetTransactionResponseData`](https://github.com/multiversx/mx-chain-proxy-go/blob/master/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransactionResponseData {
    pub transaction: ApiTransactionResult,
}

/// Corresponds to [`GetTransactionResponse`](https://github.com/multiversx/mx-chain-proxy-go/blob/master/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransactionResponse {
    #[serde(default)]
    pub error: String,
    pub code: String,
    pub data: Option<GetTransactionResponseData>,
}
