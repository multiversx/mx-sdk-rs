use multiversx_chain_core::std::Bech32Address;
use serde::{Deserialize, Serialize};

/// Entry of the upgrade history of a contract (`/accounts/{address}/upgrades`), newest first.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiUpgrade {
    /// The account that performed the upgrade.
    pub address: Option<Bech32Address>,
    pub tx_hash: String,

    /// Hash of the new contract code (blake2b-256), base64-encoded.
    pub code_hash: Option<String>,
    pub timestamp: u64,
}
