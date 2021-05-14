elrond_wasm::imports!();

/// Contains all events that can be emitted by the contract.
#[elrond_wasm_derive::module]
pub trait BlockchainApiFeatures {
	#[endpoint(get_caller)]
	fn get_caller_endpoint(&self) -> Address {
		self.blockchain().get_caller()
	}

	#[endpoint(get_shard_of_address)]
	fn get_shard_of_address_endpoint(&self, address: &Address) -> u32 {
		self.blockchain().get_shard_of_address(address)
	}

	#[endpoint(is_smart_contract)]
	fn is_smart_contract_endpoint(&self, address: &Address) -> bool {
		self.blockchain().is_smart_contract(address)
	}

	#[endpoint(get_owner_address)]
	fn get_owner_address_endpoint(&self) -> Address {
		self.blockchain().get_owner_address()
	}

	#[endpoint(get_gas_left)]
	fn get_gas_left_endpoint(&self) -> u64 {
		self.blockchain().get_gas_left()
	}
}
