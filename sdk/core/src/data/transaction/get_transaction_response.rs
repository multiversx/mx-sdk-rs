use serde::{Deserialize, Serialize};

use super::api_transaction_result::ApiTransactionResult;

/// Follows the format of the data field of a get transaction response.
///
/// Corresponds to [`GetTransactionResponseData`](https://github.com/multiversx/mx-chain-proxy-go/blob/master/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransactionResponseData {
    pub transaction: ApiTransactionResult,
}

/// Defines a response from the node holding the transaction sent from the chain.
///
/// Corresponds to [`GetTransactionResponse`](https://github.com/multiversx/mx-chain-proxy-go/blob/master/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransactionResponse {
    #[serde(default)]
    pub error: String,
    pub code: String,
    pub data: Option<GetTransactionResponseData>,
}
