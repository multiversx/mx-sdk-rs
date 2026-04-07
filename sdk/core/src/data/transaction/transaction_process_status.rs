use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionProcessStatusData {
    pub reason: String,
    pub status: String,
}

// TransactionProcessStatus holds a transaction's status response from the network obtained through the process-status API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionProcessStatus {
    pub error: String,
    pub code: String,
    pub data: Option<TransactionProcessStatusData>,
}
