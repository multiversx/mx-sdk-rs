use serde::{Deserialize, Serialize};

/// Simplified decode of [`TxCostResponseData`](https://github.com/multiversx/mx-chain-proxy-go/blob/master/data/transaction.go)
/// used when only the gas cost is needed from the `/transaction/cost` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateGasTransactionData {
    pub tx_gas_units: u64,
}

/// Simplified response envelope for the `/transaction/cost` endpoint when only gas units are needed.
/// For the full response, use [`ResponseTxCost`](super::tx_cost::ResponseTxCost).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulateGasTransactionResponse {
    pub error: String,
    pub code: String,
    pub data: Option<SimulateGasTransactionData>,
}
