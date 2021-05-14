elrond_wasm::imports!();

/// Contains all events that can be emitted by the contract.
#[elrond_wasm_derive::module]
pub trait BlockchainApiFeatures {
	#[endpoint]
	fn get_caller(&self) -> Address {
		self.blockchain().get_caller()
	}

	#[endpoint]
	fn get_shard_of_address(&self, address: &Address) -> u32 {
		self.blockchain().get_shard_of_address(address)
	}

	#[endpoint]
	fn is_smart_contract(&self, address: &Address) -> bool {
		self.blockchain().is_smart_contract(address)
	}

	#[endpoint]
	fn get_owner_address(&self) -> Address {
		self.blockchain().get_owner_address()
	}

	#[endpoint]
	fn get_gas_left(&self) -> u64 {
		self.blockchain().get_gas_left()
	}

	#[endpoint]
	fn get_cumulated_validator_rewards(&self) -> Self::BigUint {
		self.blockchain().get_cumulated_validator_rewards()
	}

	#[endpoint]
	fn get_esdt_local_roles(&self, token_id: TokenIdentifier) -> MultiResultVec<BoxedBytes> {
		let roles = self
			.blockchain()
			.get_esdt_local_roles(token_id.as_esdt_identifier());
		let role_names: Vec<BoxedBytes> = roles
			.iter()
			.map(|role| BoxedBytes::from(role.as_role_name()))
			.collect();
		role_names.into()
	}
}
