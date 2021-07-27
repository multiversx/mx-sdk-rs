#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait SecondContract {
	#[init]
	fn init(&self, esdt_token_name: TokenIdentifier) {
		self.set_contract_esdt_token_name(&esdt_token_name);
	}

	#[payable("*")]
	#[endpoint(acceptEsdtPayment)]
	fn accept_esdt_payment(
		&self,
		#[payment_token] actual_token_name: TokenIdentifier,
	) -> SCResult<()> {
		let expected_token_name = self.get_contract_esdt_token_name();
		require!(actual_token_name == expected_token_name, "Wrong esdt token");
		Ok(())
	}

	#[payable("*")]
	#[endpoint(rejectEsdtPayment)]
	fn reject_esdt_payment(&self) -> SCResult<()> {
		sc_error!("Rejected")
	}

	// storage

	#[storage_set("esdtTokenName")]
	fn set_contract_esdt_token_name(&self, esdt_token_name: &TokenIdentifier);

	#[view(getEsdtTokenName)]
	#[storage_get("esdtTokenName")]
	fn get_contract_esdt_token_name(&self) -> TokenIdentifier;
}
