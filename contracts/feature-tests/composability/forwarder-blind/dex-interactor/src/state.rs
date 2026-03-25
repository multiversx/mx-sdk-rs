use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

/// State file
const STATE_FILE: &str = "state.toml";

/// ForwarderBlind Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    #[serde(default)]
    contract_addresses: Vec<Bech32Address>,
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
        self.contract_addresses = addresses;
    }

    pub fn contract_addresses(&self) -> &[Bech32Address] {
        &self.contract_addresses
    }
}

impl Drop for State {
    // Serializes state to file
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        let mut content = String::from("contract_addresses = [\n");
        for addr in &self.contract_addresses {
            content.push_str(&format!("    \"{addr}\",\n"));
        }
        content.push_str("]\n");
        file.write_all(content.as_bytes()).unwrap();
    }
}
