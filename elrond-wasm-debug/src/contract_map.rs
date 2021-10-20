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
            panic!(
                "Unknown contract: {}",
                std::str::from_utf8(contract_identifier).unwrap()
            );
        }
    }

    pub fn register_contract(
        &mut self,
        path: &str,
        new_contract_closure: Box<dyn Fn(DebugApi) -> Box<dyn CallableContract<A>>>,
    ) {
        self.factories
            .insert(path.as_bytes().to_vec(), new_contract_closure);
    }
}

impl<A> Default for ContractMap<A> {
    fn default() -> Self {
        Self::new()
    }
}
