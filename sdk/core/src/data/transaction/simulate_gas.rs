use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateGasTransactionData {
    pub tx_gas_units: u64,
}

// SimulateGasTransactionResponse holds the response received from the network when estimating cost of a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulateGasTransactionResponse {
    pub error: String,
    pub code: String,
    pub data: Option<SimulateGasTransactionData>,
}
