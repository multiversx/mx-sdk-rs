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
pub type ForwarderQueueContract = ContractInfo<forwarder_queue::Proxy<DebugApi>>;
pub type PromisesContract = ContractInfo<promises_features::Proxy<DebugApi>>;

/// Composability Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    vault_address: Option<String>,
    forwarder_queue_address: Option<String>,
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
    /// Sets the forwarder-queue address
    pub fn set_forwarder_queue_address(&mut self, address: &str) {
        self.forwarder_queue_address = Some(String::from(address));
    }
    /// Sets the promises address
    pub fn set_promises_address(&mut self, address: &str) {
        self.promises_address = Some(String::from(address));
    }

    /// Returns the vault contract
    pub fn vault_from_address(&self, address: &str) -> VaultContract {
        VaultContract::new(address)
    }

    /// Returns the forwarder-queue contract
    pub fn forwarder_queue_from_addr(&self, address: &str) -> ForwarderQueueContract {
        ForwarderQueueContract::new(address)
    }

    /// Returns the promises contract
    pub fn _promises(&self) -> PromisesContract {
        PromisesContract::new(self.promises_address.clone().unwrap())
    }

    /// Returns the vault contract with default address
    pub fn default_vault_address(&self) -> VaultContract {
        VaultContract::new(DEFAULT_CONTRACT_ADDRESS)
    }

    /// Returns the forwarder-queue contract with default address
    pub fn default_forwarder_queue_address(&self) -> ForwarderQueueContract {
        ForwarderQueueContract::new(DEFAULT_CONTRACT_ADDRESS)
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
