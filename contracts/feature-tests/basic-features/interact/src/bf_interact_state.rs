use multiversx_sc_snippets::imports::Bech32Address;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

/// State file
const STATE_FILE: &str = "state.toml";

/// Basic Features Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    bf_address_storage_bytes: Option<Bech32Address>,
    bf_address: Option<Bech32Address>,
    bf_address_crypto: Option<Bech32Address>,
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

    /// Sets the contract address for basic-features-storage-bytes
    pub fn set_bf_address_storage_bytes(&mut self, address: Bech32Address) {
        self.bf_address_storage_bytes = Some(address);
    }

    /// Sets the contract address for basic-features-crypto
    pub fn set_bf_address_crypto(&mut self, address: Bech32Address) {
        self.bf_address_crypto = Some(address);
    }

    /// Sets the contract address for basic-features
    pub fn set_bf_address(&mut self, address: Bech32Address) {
        self.bf_address = Some(address);
    }

    /// Returns basic-features-storage-bytes contract
    pub fn bf_storage_bytes_contract(&self) -> &Bech32Address {
        self.bf_address_storage_bytes
            .as_ref()
            .expect("basic-features-storage-bytes contract not yet deployed")
    }

    /// Returns basic-features-storage-bytes contract
    pub fn bf_crypto_contract(&self) -> &Bech32Address {
        self.bf_address_crypto
            .as_ref()
            .expect("basic-features-crypto contract not yet deployed")
    }

    /// Returns basic-features contract
    pub fn bf_contract(&self) -> &Bech32Address {
        self.bf_address
            .as_ref()
            .expect("basic-features contract not yet deployed")
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
