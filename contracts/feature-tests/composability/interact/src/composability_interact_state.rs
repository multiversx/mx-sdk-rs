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
pub type ForwarderRawContract = ContractInfo<forwarder_queue::Proxy<DebugApi>>;
pub type PromisesContract = ContractInfo<promises_features::Proxy<DebugApi>>;

/// Composability Interact Contract
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InteractionContract {
    pub own_address: Option<String>,
    pub child_contracts: Vec<InteractionContract>,
}

/// Composability Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub root_contract: InteractionContract,
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

    // /// Sets the forwarder-queue address
    // pub fn set_root_contract_address(&mut self, address: &str) {
    //     self.root_contract.own_address = Some(String::from(address));
    // }

    // /// Sets the forwarder-queue address
    // pub fn get_root_contract(&mut self) -> InteractionContract {
    //     self.root_contract
    // }

    /// Sets the forwarder-queue address
    pub fn get_child_contracts(
        &mut self,
        contract: InteractionContract,
    ) -> Vec<InteractionContract> {
        contract.child_contracts
    }

    /// Sets the forwarder-queue address
    pub fn get_own_address(&mut self, contract: InteractionContract) -> String {
        contract.own_address.unwrap()
    }

    /// Returns the vault contract with default address
    pub fn default_vault_address(&self) -> VaultContract {
        VaultContract::new(DEFAULT_CONTRACT_ADDRESS)
    }

    /// Returns the forwarder-queue contract with default address
    pub fn default_forwarder_queue_address(&self) -> ForwarderRawContract {
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

impl InteractionContract {
    pub fn new() -> Self {

    }
}