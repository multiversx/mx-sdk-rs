use crate::{ContractInfo, StaticApi};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

const DEFAULT_CONTRACT_ADDRESS: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";

/// State file
const STATE_FILE: &str = "state.toml";

pub type BasicFeaturesContract = ContractInfo<basic_features::Proxy<StaticApi>>;

/// Multisig Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    bf_address: Option<String>,
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

    /// Sets the contract address
    pub fn set_bf_address(&mut self, address: &str) {
        self.bf_address = Some(String::from(address));
    }

    /// Returns the contract
    pub fn bf_contract(&self) -> BasicFeaturesContract {
        BasicFeaturesContract::new(
            self.bf_address
                .clone()
                .expect("basic-features contract not yet deployed"),
        )
    }

    /// Returns the adder contract with default address
    pub fn default_contract(&self) -> BasicFeaturesContract {
        BasicFeaturesContract::new(DEFAULT_CONTRACT_ADDRESS)
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
