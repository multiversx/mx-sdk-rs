use serde::{Deserialize, Serialize};

/// Transaction as returned by the MultiversX API (`/transactions/{hash}`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiTransaction {
    pub tx_hash: Option<String>,
    pub status: String,
    pub timestamp: u64,

    /// Transaction data field, base64-encoded.
    pub data: Option<String>,
}
