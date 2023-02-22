use crate::{ContractInfo, DebugApi};
use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

/// Default multisig address expr if None is set
const DEFAULT_MULTISIG_ADDRESS_EXPR: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";

/// State file
const STATE_FILE: &str = "state.toml";

type MultisigContract = ContractInfo<multisig::Proxy<DebugApi>>;

/// Multisig Interact state
#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    multisig_address: Option<String>,
}

impl State {
    // Loads state from file and deserializes it
    // Creates file if it doesn't exist
    pub fn load_state() -> Self {
        let mut file =
            std::fs::File::open(STATE_FILE).unwrap_or(std::fs::File::create(STATE_FILE).unwrap());
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        toml::from_str(&content).unwrap()
    }

    /// Sets the multisig address
    pub fn set_multisig_address(&mut self, address: &str) {
        self.multisig_address = Some(String::from(address));
    }

    /// Returns the multisig contract
    pub fn multisig(&self) -> MultisigContract {
        match &self.multisig_address {
            Some(address) => MultisigContract::new(address.clone()),
            None => MultisigContract::new(DEFAULT_MULTISIG_ADDRESS_EXPR),
        }
    }
}

impl Drop for State {
    // Serializes state to file on drop
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}
