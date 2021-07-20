elrond_wasm::imports!();

#[elrond_wasm_derive::module]
pub trait DeployContractModule {
	#[endpoint(deployContract)]
	fn deploy_contract(
		&self,
		code: BoxedBytes,
		#[var_args] arguments: VarArgs<BoxedBytes>,
	) -> SCResult<Address> {
		let deployed_contract_address = self
			.deploy(&code, &arguments.into_vec())
			.ok_or("Deploy failed")?;

		Ok(deployed_contract_address)
	}

	#[endpoint(deployTwoContracts)]
	fn deploy_two_contracts(
		&self,
		code: BoxedBytes,
		#[var_args] arguments: VarArgs<BoxedBytes>,
	) -> SCResult<(Address, Address)> {
		let args_as_vec = arguments.into_vec();
		let first_deployed_contract_address = self
			.deploy(&code, &args_as_vec)
			.ok_or("First deploy failed")?;

		let second_deployed_contract_address = self
			.deploy(&code, &args_as_vec)
			.ok_or("Second deploy failed")?;

		Ok((
			first_deployed_contract_address,
			second_deployed_contract_address,
		))
	}

	fn deploy(&self, code: &BoxedBytes, arguments: &[BoxedBytes]) -> Option<Address> {
		self.send().deploy_contract(
			self.blockchain().get_gas_left(),
			&Self::BigUint::zero(),
			code,
			CodeMetadata::DEFAULT,
			&arguments.into(),
		)
	}
}
