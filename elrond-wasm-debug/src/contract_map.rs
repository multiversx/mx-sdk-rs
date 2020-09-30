use super::*;

use elrond_wasm::CallableContract;

use alloc::boxed::Box;
use alloc::vec::Vec;
use std::collections::HashMap;

pub struct ContractMap {
    factories: HashMap<Vec<u8>, Box<dyn Fn(ArwenMockRef) -> Box<dyn CallableContract>>>,
}

impl ContractMap {
    pub fn new() -> Self {
        ContractMap{
            factories: HashMap::new()
        }
    }

    pub fn new_contract_instance(&self,
        contract_identifier: &Vec<u8>,
        mock_ref: &ArwenMockRef,
    ) -> Box<dyn CallableContract> {

        if let Some(new_contract_closure) = self.factories.get(contract_identifier) {
            new_contract_closure(mock_ref.clone())
        } else {
            panic!("Unknown contract");
        }
    }

    pub fn register_contract(&mut self,
        path: &str,
        new_contract_closure: Box<dyn Fn(ArwenMockRef) -> Box<dyn CallableContract>>) {

        self.factories.insert(path.as_bytes().to_vec(), new_contract_closure);
    }
}
