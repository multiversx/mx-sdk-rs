use crate::DebugApi;
use crate::ContractInfo;
use std::io::{Read, Write};

use serde::{Serialize, Deserialize};

const DEFAULT_MULTISIG_ADDRESS_EXPR: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";
const STATE_FILE_NAME: &str = "state.toml";

type MultisigContract = ContractInfo<multisig::Proxy<DebugApi>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    multisig_address: Option<String>,
}

impl State {
    pub fn load_state() -> Self {
        let mut file = std::fs::File::open(STATE_FILE_NAME)
            .unwrap_or(std::fs::File::create(STATE_FILE_NAME).unwrap());
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        toml::from_str(&content).unwrap()
    }

    pub fn set_multisig_address(&mut self, address: &str) {
        self.multisig_address = Some(String::from(address));
    }

    pub fn multisig(&self) -> MultisigContract {
        match &self.multisig_address {
            Some(address) => MultisigContract::new(address.clone()),
            None => MultisigContract::new(DEFAULT_MULTISIG_ADDRESS_EXPR),
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE_NAME).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}