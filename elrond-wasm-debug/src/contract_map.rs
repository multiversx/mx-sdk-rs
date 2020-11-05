use super::*;

use elrond_wasm::CallableContract;

use alloc::boxed::Box;
use alloc::vec::Vec;
use std::collections::HashMap;

pub struct ContractMap<A> {
	factories: HashMap<Vec<u8>, Box<dyn Fn(TxContext) -> Box<dyn CallableContract<A>>>>,
}

impl<A> ContractMap<A> {
	pub fn new() -> Self {
		ContractMap { factories: HashMap::new() }
	}

	pub fn new_contract_instance(&self, contract_identifier: &Vec<u8>, tx_context: TxContext) -> Box<dyn CallableContract<A>> {
		if let Some(new_contract_closure) = self.factories.get(contract_identifier) {
			new_contract_closure(tx_context)
		} else {
			panic!("Unknown contract");
		}
	}

	pub fn register_contract(&mut self, path: &str, new_contract_closure: Box<dyn Fn(TxContext) -> Box<dyn CallableContract<A>>>) {
		self.factories.insert(path.as_bytes().to_vec(), new_contract_closure);
	}
}
