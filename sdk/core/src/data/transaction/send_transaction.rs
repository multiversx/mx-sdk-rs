use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransactionData {
    pub tx_hash: String,
}

// SendTransactionResponse holds the response received from the network when broadcasting a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionResponse {
    pub error: String,
    pub code: String,
    pub data: Option<SendTransactionData>,
}
