use crate::DebugApi;

use super::*;

use alloc::vec::Vec;
use std::{collections::HashMap, fmt};

pub struct ContractMap {
    contract_objs: HashMap<Vec<u8>, ContractContainer>,
}

impl fmt::Debug for ContractMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContractMap").finish()
    }
}

impl ContractMap {
    pub fn new() -> Self {
        ContractMap {
            contract_objs: HashMap::new(),
        }
    }

    pub fn get_contract(
        &self,
        contract_identifier: &[u8],
        _debug_api: DebugApi,
    ) -> &ContractContainer {
        if let Some(contract_contatiner) = self.contract_objs.get(contract_identifier) {
            contract_contatiner
        } else {
            unknown_contract_panic(contract_identifier)
        }
    }

    pub fn register_contract(
        &mut self,
        contract_bytes: Vec<u8>,
        contract_container: ContractContainer,
    ) {
        let previous_entry = self
            .contract_objs
            .insert(contract_bytes, contract_container);
        assert!(previous_entry.is_none(), "contract inserted twice");
    }

    pub fn contains_contract(&self, contract_bytes: &[u8]) -> bool {
        self.contract_objs.contains_key(contract_bytes)
    }
}

fn unknown_contract_panic(contract_identifier: &[u8]) -> ! {
    if let Ok(s) = std::str::from_utf8(contract_identifier) {
        panic!("Unknown contract: {s}")
    } else {
        panic!(
            "Unknown contract of length {} bytes",
            contract_identifier.len()
        )
    }
}

impl Default for ContractMap {
    fn default() -> Self {
        Self::new()
    }
}
