#![no_std]
#![allow(unused_attributes)]

elrond_wasm::imports!();

#[elrond_wasm_derive::contract]
pub trait DeployTwoContracts {
	#[init]
	fn init(&self) {}

	#[endpoint(deployContract)]
	fn deploy_contract(&self, code: BoxedBytes) -> SCResult<Address> {
		let deployed_contract_address = self.deploy(&code);
		if deployed_contract_address.is_zero() {
			return sc_error!("Deploy failed");
		}

		Ok(deployed_contract_address)
	}

	#[endpoint(deployTwoContracts)]
	fn deploy_two_contracts(&self, code: BoxedBytes) -> SCResult<(Address, Address)> {
		let first_deployed_contract_address = self.deploy(&code);
		if first_deployed_contract_address.is_zero() {
			return sc_error!("First deploy failed");
		}

		let second_deployed_contract_address = self.deploy(&code);
		if second_deployed_contract_address.is_zero() {
			return sc_error!("Second deploy failed");
		}

		Ok((
			first_deployed_contract_address,
			second_deployed_contract_address,
		))
	}

	#[endpoint(upgradeContract)]
	fn upgrade_contract(&self, child_sc_address: Address, new_code: BoxedBytes) {
		self.upgrade(&child_sc_address, &new_code);
	}

	#[endpoint(changeOwnerAddress)]
	fn change_owner(&self, child_sc_address: Address, new_owner: Address) {
		self.send().change_owner_address(&child_sc_address, &new_owner);
	}

	fn upgrade(&self, child_sc_address: &Address, new_code: &BoxedBytes) {
		self.send().upgrade_contract(
			child_sc_address,
			self.blockchain().get_gas_left(),
			&Self::BigUint::zero(),
			new_code,
			CodeMetadata::DEFAULT,
			ArgBuffer::new(),
		);
	}

	fn deploy(&self, code: &BoxedBytes) -> Address {
		self.send().deploy_contract(
			self.blockchain().get_gas_left(),
			&Self::BigUint::zero(),
			code,
			CodeMetadata::DEFAULT,
			&ArgBuffer::new(),
		)
	}
}
