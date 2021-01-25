#![no_std]
#![allow(unused_attributes)]

imports!();

#[elrond_wasm_derive::contract(SecondContractImpl)]
pub trait SecondContract {
	#[init]
	fn init(&self, esdt_token_name: BoxedBytes) {
		self.set_contract_esdt_token_name(&esdt_token_name);
	}

	#[endpoint(acceptEsdtPayment)]
	fn accept_esdt_payment(&self) -> SCResult<()> {
		let expected_token_name = self.get_contract_esdt_token_name();
		let actual_token_name = self.get_esdt_token_name_boxed();

		require!(actual_token_name == expected_token_name, "Wrong esdt token");

		Ok(())
	}

	#[endpoint(rejectEsdtPayment)]
	fn reject_esdt_payment(&self) -> SCResult<()> {
		sc_error!("Rejected")
	}

	fn get_esdt_token_name_boxed(&self) -> BoxedBytes {
		BoxedBytes::from(self.get_esdt_token_name())
	}

	// storage

	#[storage_set("esdtTokenName")]
	fn set_contract_esdt_token_name(&self, esdt_token_name: &BoxedBytes);

	#[view(getEsdtTokenName)]
	#[storage_get("esdtTokenName")]
	fn get_contract_esdt_token_name(&self) -> BoxedBytes;
}
