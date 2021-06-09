#![no_std]
#![allow(unused_attributes)]

elrond_wasm::imports!();

#[elrond_wasm_derive::contract]
pub trait DeployTwoContracts {
	#[init]
	fn init(&self) {}

	#[endpoint(deployContract)]
	fn deploy_contract(
		&self,
		code: BoxedBytes,
		#[var_args] arguments: VarArgs<BoxedBytes>,
	) -> SCResult<Address> {
		let deployed_contract_address = self.deploy(&code, &arguments.into_vec());
		if deployed_contract_address.is_zero() {
			return sc_error!("Deploy failed");
		}

		Ok(deployed_contract_address)
	}

	#[endpoint(deployTwoContracts)]
	fn deploy_two_contracts(
		&self,
		code: BoxedBytes,
		#[var_args] arguments: VarArgs<BoxedBytes>,
	) -> SCResult<(Address, Address)> {
		let args_as_vec = arguments.into_vec();
		let first_deployed_contract_address = self.deploy(&code, &args_as_vec);
		if first_deployed_contract_address.is_zero() {
			return sc_error!("First deploy failed");
		}

		let second_deployed_contract_address = self.deploy(&code, &args_as_vec);
		if second_deployed_contract_address.is_zero() {
			return sc_error!("Second deploy failed");
		}

		Ok((
			first_deployed_contract_address,
			second_deployed_contract_address,
		))
	}

	#[endpoint(upgradeContract)]
	fn upgrade_contract(
		&self,
		child_sc_address: Address,
		new_code: BoxedBytes,
		#[var_args] arguments: VarArgs<BoxedBytes>,
	) {
		self.upgrade(&child_sc_address, &new_code, &arguments.into_vec());
	}

	#[endpoint(changeOwnerAddress)]
	fn change_owner(&self, child_sc_address: Address, new_owner: Address) {
		self.send()
			.change_owner_address(&child_sc_address, &new_owner);
	}

	fn upgrade(&self, child_sc_address: &Address, new_code: &BoxedBytes, arguments: &[BoxedBytes]) {
		self.send().upgrade_contract(
			child_sc_address,
			self.blockchain().get_gas_left(),
			&Self::BigUint::zero(),
			new_code,
			CodeMetadata::DEFAULT,
			&self.build_arg_buffer(arguments),
		);
	}

	fn deploy(&self, code: &BoxedBytes, arguments: &[BoxedBytes]) -> Address {
		self.send().deploy_contract(
			self.blockchain().get_gas_left(),
			&Self::BigUint::zero(),
			code,
			CodeMetadata::DEFAULT,
			&self.build_arg_buffer(arguments),
		)
	}

	fn build_arg_buffer(&self, arguments: &[BoxedBytes]) -> ArgBuffer {
		let mut arg_buffer = ArgBuffer::new();
		for arg in arguments {
			arg_buffer.push_argument_bytes(arg.as_slice());
		}

		arg_buffer
	}
}
