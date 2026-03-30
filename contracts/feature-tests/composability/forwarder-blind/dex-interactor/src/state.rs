use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{io::Read, path::Path};

/// State file
const STATE_FILE: &str = "deploy.toml";

/// ForwarderBlind Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    #[serde(default)]
    last_deployed: Vec<Bech32Address>,
}

impl State {
    // Deserializes state from file
    pub fn load_state() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

    pub fn set_contract_addresses(&mut self, addresses: Vec<Bech32Address>) {
        self.last_deployed = addresses;
    }

    pub fn contract_addresses(&self) -> &[Bech32Address] {
        &self.last_deployed
    }
}

impl Drop for State {
    // Serializes state to file
    fn drop(&mut self) {
        let content = format!(
            "# These are the last deployed addresses. Copy them to config.toml contract_addresses to use them.\n{}",
            toml::to_string_pretty(self).unwrap()
        );
        std::fs::write(STATE_FILE, content).unwrap();
    }
}
