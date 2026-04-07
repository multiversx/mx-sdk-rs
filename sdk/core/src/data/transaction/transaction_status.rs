use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStatusData {
    pub status: String,
}

// TransactionStatus holds a transaction's status response from the network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStatus {
    pub error: String,
    pub code: String,
    pub data: Option<TransactionStatusData>,
}
