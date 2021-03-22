use super::*;

use elrond_wasm::api::CallableContract;

use alloc::boxed::Box;
use alloc::vec::Vec;
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
			panic!("Unknown contract");
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
