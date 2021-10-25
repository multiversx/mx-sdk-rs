use crate::DebugApi;

use super::*;

use alloc::{boxed::Box, vec::Vec};
use elrond_wasm::contract_base::CallableContract;
use std::{collections::HashMap, fmt};

pub type ContractCallFactory<A> = Box<dyn Fn(DebugApi) -> Box<dyn CallableContract<A>>>;

pub struct ContractMap<A> {
    factories: HashMap<Vec<u8>, ContractCallFactory<A>>,
}

impl<A> fmt::Debug for ContractMap<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContractMap").finish()
    }
}

impl<A> ContractMap<A> {
    pub fn new() -> Self {
        ContractMap {
            factories: HashMap::new(),
        }
    }

    pub fn new_contract_instance(
        &self,
        contract_identifier: &[u8],
        debug_api: DebugApi,
    ) -> Box<dyn CallableContract<A>> {
        if let Some(new_contract_closure) = self.factories.get(contract_identifier) {
            new_contract_closure(debug_api)
        } else {
            unknown_contract_panic(contract_identifier)
        }
    }

    pub fn register_contract(
        &mut self,
        contract_bytes: Vec<u8>,
        new_contract_closure: Box<dyn Fn(DebugApi) -> Box<dyn CallableContract<A>>>,
    ) {
        let previous_entry = self.factories.insert(contract_bytes, new_contract_closure);
        assert!(previous_entry.is_none(), "contract inserted twice");
    }
}

fn unknown_contract_panic(contract_identifier: &[u8]) -> ! {
    if let Ok(s) = std::str::from_utf8(contract_identifier) {
        panic!("Unknown contract: {}", s)
    } else {
        panic!(
            "Unknown contract of length {} bytes",
            contract_identifier.len()
        )
    }
}

impl<A> Default for ContractMap<A> {
    fn default() -> Self {
        Self::new()
    }
}
