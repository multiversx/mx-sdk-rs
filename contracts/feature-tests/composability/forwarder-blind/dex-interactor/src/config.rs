use multiversx_sc_snippets::imports::Bech32Address;
use serde::Deserialize;
use std::io::Read;

/// Config file
const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChainType {
    Real,
    Simulator,
}

/// Contract Interact configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    pub gateway_uri: String,
    pub chain_type: ChainType,
    pub wegld_address: Bech32Address,
    pub pair_address: Bech32Address,
    pub wegld_token_id: String,
    pub usdc_token_id: String,
    /// Optional list of PEM file paths, one per wallet.
    /// If absent or empty, the built-in test wallets (sophie/simon/szonja) are used.
    #[serde(default)]
    pub wallet_pem_paths: Vec<String>,
    /// Forwarder contract addresses to target for all swap transactions.
    #[serde(default)]
    pub contract_addresses: Vec<Bech32Address>,
}

impl Config {
    // Deserializes config from file
    pub fn load_config() -> Self {
        let mut file = std::fs::File::open(CONFIG_FILE).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        toml::from_str(&content).unwrap()
    }

    // Returns the gateway URI
    pub fn gateway_uri(&self) -> &str {
        &self.gateway_uri
    }

    // Returns if chain type is chain simulator
    pub fn use_chain_simulator(&self) -> bool {
        match self.chain_type {
            ChainType::Real => false,
            ChainType::Simulator => true,
        }
    }
}
