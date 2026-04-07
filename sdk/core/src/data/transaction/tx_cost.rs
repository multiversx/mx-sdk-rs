use serde::{Deserialize, Serialize};

// TxCostResponseData follows the format of the data field of a transaction cost request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxCostResponseData {
    pub tx_gas_units: u64,
    pub return_message: String,
}

// TxCostResponse defines a response from the node holding the transaction cost
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxCostResponse {
    pub data: Option<TxCostResponseData>,
    pub error: String,
    pub code: String,
}
