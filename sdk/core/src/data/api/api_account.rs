use multiversx_chain_core::std::Bech32Address;
use serde::{Deserialize, Serialize};

/// Account as returned by the MultiversX API (`/accounts/{address}`).
///
/// Unlike the gateway, the API returns the response unwrapped, without a `data` envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiAccount {
    pub address: Bech32Address,

    /// Contract code, hex-encoded.
    pub code: Option<String>,

    /// Hash of the contract code (blake2b-256), base64-encoded.
    pub code_hash: Option<String>,
    pub deploy_tx_hash: Option<String>,

    /// Deploy timestamp, in seconds.
    pub deployed_at: Option<u64>,
    pub owner_address: Option<Bech32Address>,
}
