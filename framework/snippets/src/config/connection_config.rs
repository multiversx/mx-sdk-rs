use serde::Deserialize;

use super::ChainType;

/// Configuration for connecting to the MultiversX network.
#[derive(Debug, Deserialize)]
pub struct ConnectionConfig {
    pub gateway_uri: String,
    pub chain_type: ChainType,
}

impl ConnectionConfig {
    /// Creates a default chain-simulator connection config pointing to localhost.
    pub fn chain_simulator() -> Self {
        ConnectionConfig {
            gateway_uri: "http://localhost:8085".to_owned(),
            chain_type: ChainType::Simulator,
        }
    }

    pub fn gateway_uri(&self) -> &str {
        &self.gateway_uri
    }

    pub fn use_chain_simulator(&self) -> bool {
        matches!(self.chain_type, ChainType::Simulator)
    }
}
