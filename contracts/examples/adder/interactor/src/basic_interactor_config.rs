use multiversx_sc_snippets::{ConnectionConfig, WalletConfig};
use serde::Deserialize;
use std::io::Read;

/// Config file
const CONFIG_FILE: &str = "config.toml";

/// Adder Interact configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    pub contract_path: String,
    pub connection: ConnectionConfig,
    pub owner: WalletConfig,
    pub wallet: WalletConfig,
}

impl Config {
    // Deserializes config from file
    pub fn load_config() -> Self {
        let mut file = std::fs::File::open(CONFIG_FILE).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        toml::from_str(&content).unwrap()
    }

    pub fn chain_simulator_config() -> Self {
        Config {
            contract_path: "../output/adder.mxsc.json".to_owned(),
            connection: ConnectionConfig::chain_simulator(),
            owner: WalletConfig::from_test_wallet("mike"),
            wallet: WalletConfig::from_test_wallet("ivan"),
        }
    }
}
