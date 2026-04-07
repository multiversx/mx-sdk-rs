use serde::{Deserialize, Serialize};

use super::transaction_on_network::TransactionOnNetwork;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfoData {
    pub transaction: TransactionOnNetwork,
}

// TransactionInfo holds a transaction info response from the network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    #[serde(default)]
    pub error: String,
    pub code: String,
    pub data: Option<TransactionInfoData>,
}
