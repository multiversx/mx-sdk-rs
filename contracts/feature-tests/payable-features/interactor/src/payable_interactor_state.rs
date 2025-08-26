use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

/// State file
const STATE_FILE: &str = "state.toml";

/// Payable Features Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    payable_features_address: Option<Bech32Address>,
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

    /// Sets the payable features address
    pub fn set_payable_features_address(&mut self, address: Bech32Address) {
        self.payable_features_address = Some(address);
    }

    /// Returns the payable features contract
    pub fn current_payable_features_address(&self) -> &Bech32Address {
        self.payable_features_address
            .as_ref()
            .expect("no known payable features contract, deploy first")
    }
}

impl Drop for State {
    // Serializes state to file
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}
