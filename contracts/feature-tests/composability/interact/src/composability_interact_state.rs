use crate::{ContractInfo, DebugApi};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

/// Default contract address
const DEFAULT_CONTRACT_ADDRESS: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";

/// State file
const STATE_FILE: &str = "state.toml";

pub type VaultContract = ContractInfo<vault::Proxy<DebugApi>>;
pub type ForwarderRawContract = ContractInfo<forwarder_raw::Proxy<DebugApi>>;
pub type PromisesContract = ContractInfo<promises_features::Proxy<DebugApi>>;

/// Composability Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    vault_address: Option<String>,
    forwarder_raw_address: Option<String>,
    promises_address: Option<String>,
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

    /// Sets the vault address
    pub fn set_vault_address(&mut self, address: &str) {
        self.vault_address = Some(String::from(address));
    }
    /// Sets the forwarder-raw address
    pub fn set_forwarder_raw_address(&mut self, address: &str) {
        self.forwarder_raw_address = Some(String::from(address));
    }
    /// Sets the promises address
    pub fn set_promises_address(&mut self, address: &str) {
        self.promises_address = Some(String::from(address));
    }

    /// Returns the vault contract
    pub fn vault(&self) -> VaultContract {
        VaultContract::new(self.vault_address.clone().unwrap())
    }

    /// Returns the forwarder-raw contract
    pub fn forwarder_raw(&self) -> ForwarderRawContract {
        ForwarderRawContract::new(self.forwarder_raw_address.clone().unwrap())
    }

    /// Returns the promises contract
    pub fn _promises(&self) -> PromisesContract {
        PromisesContract::new(self.promises_address.clone().unwrap())
    }

    /// Returns the vault contract with default address
    pub fn default_vault_address(&self) -> VaultContract {
        VaultContract::new(DEFAULT_CONTRACT_ADDRESS)
    }

    /// Returns the forwarder-raw contract with default address
    pub fn default_forwarder_raw_address(&self) -> ForwarderRawContract {
        ForwarderRawContract::new(DEFAULT_CONTRACT_ADDRESS)
    }

    /// Returns the promises contract with default address
    pub fn default_promises_address(&self) -> PromisesContract {
        PromisesContract::new(DEFAULT_CONTRACT_ADDRESS)
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
