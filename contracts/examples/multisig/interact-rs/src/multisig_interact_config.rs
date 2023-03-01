use serde::Deserialize;
use std::io::Read;

/// Config file
const CONFIG_FILE: &str = "config.toml";

/// Multisig Interact configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    gateway: String,
    alice_pem: String,
    bob_pem: String,
}

impl Config {
    // Deserializes config from file
    pub fn load_config() -> Self {
        let mut file = std::fs::File::open(CONFIG_FILE).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        toml::from_str(&content).unwrap()
    }

    // Returns the gateway
    pub fn gateway(&self) -> &str {
        &self.gateway
    }

    // Returns alice's pem
    pub fn alice_pem(&self) -> &str {
        &self.alice_pem
    }

    // Returns bob's pem
    pub fn bob_pem(&self) -> &str {
        &self.bob_pem
    }
}
