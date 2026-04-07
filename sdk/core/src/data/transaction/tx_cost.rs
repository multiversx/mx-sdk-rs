use serde::{Deserialize, Serialize};

/// Corresponds to [`TxCostResponseData`](https://github.com/multiversx/mx-chain-proxy-go/blob/main/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxCostResponseData {
    pub tx_gas_units: u64,
    pub return_message: String,
}

/// Corresponds to [`ResponseTxCost`](https://github.com/multiversx/mx-chain-proxy-go/blob/main/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseTxCost {
    pub data: Option<TxCostResponseData>,
    pub error: String,
    pub code: String,
}
