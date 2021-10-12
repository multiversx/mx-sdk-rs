use crate::tx_mock::TxContext;

use super::*;

use alloc::{boxed::Box, vec::Vec};
use elrond_wasm::contract_base::CallableContract;
use std::collections::HashMap;

pub type ContractCallFactory<A> = Box<dyn Fn(TxContext) -> Box<dyn CallableContract<A>>>;

pub struct ContractMap<A> {
    factories: HashMap<Vec<u8>, ContractCallFactory<A>>,
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
        tx_context: TxContext,
    ) -> Box<dyn CallableContract<A>> {
        if let Some(new_contract_closure) = self.factories.get(contract_identifier) {
            new_contract_closure(tx_context)
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
        new_contract_closure: Box<dyn Fn(TxContext) -> Box<dyn CallableContract<A>>>,
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
