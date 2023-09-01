use crate::{ContractInfo, StaticApi};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

/// Default adder address
const DEFAULT_ADDER_ADDRESS: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";

/// State file
const STATE_FILE: &str = "state.toml";

pub type AdderContract = ContractInfo<adder::Proxy<StaticApi>>;

/// Multisig Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    adder_address: Option<String>,
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

    /// Sets the adder address
    pub fn set_adder_address(&mut self, address: &str) {
        self.adder_address = Some(String::from(address));
    }

    /// Returns the adder contract
    pub fn adder(&self) -> AdderContract {
        AdderContract::new(
            self.adder_address
                .clone()
                .expect("no known adder contract, deploy first"),
        )
    }

    /// Returns the adder contract with default address
    pub fn default_adder(&self) -> AdderContract {
        AdderContract::new(DEFAULT_ADDER_ADDRESS)
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
