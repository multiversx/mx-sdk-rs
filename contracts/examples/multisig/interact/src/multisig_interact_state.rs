use crate::{ContractInfo, StaticApi};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

/// Default multisig address
const DEFAULT_MULTISIG_ADDRESS: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";

/// State file
const STATE_FILE: &str = "state.toml";

pub type MultisigContract = ContractInfo<multisig::Proxy<StaticApi>>;

/// Multisig Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    multisig_address: Option<String>,
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

    /// Sets the multisig address
    pub fn set_multisig_address(&mut self, address: &str) {
        self.multisig_address = Some(String::from(address));
    }

    /// Returns the multisig contract
    pub fn multisig(&self) -> MultisigContract {
        MultisigContract::new(self.multisig_address.clone().unwrap())
    }

    /// Returns the multisig contract with default address
    pub fn default_multisig(&self) -> MultisigContract {
        MultisigContract::new(DEFAULT_MULTISIG_ADDRESS)
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
