#![no_std]

elrond_wasm::imports!();

#[elrond_wasm_derive::contract(LocalEsdtAndEsdtNftImpl)]
pub trait LocalEsdtAndEsdtNft {
	#[init]
	fn init(&self) {}

	// Fungible Tokens

	#[payable("EGLD")]
	#[endpoint(issueFungibleToken)]
	fn issue_fungible_token(
		&self,
		#[payment] issue_cost: BigUint,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
		initial_supply: BigUint,
	) -> SCResult<AsyncCall<BigUint>> {
		only_owner!(self, "only owner may call this function");

		let caller = self.get_caller();

		Ok(ESDTSystemSmartContractProxy::new()
			.issue_fungible(
				issue_cost,
				&token_display_name,
				&token_ticker,
				&initial_supply,
				0,
				false,
				false,
				false,
				true,
				true,
				true,
				true,
				true,
			)
			.async_call()
			.with_callback(self.callbacks().esdt_issue_callback(&caller)))
	}

	#[endpoint(localMint)]
	fn local_mint(&self, token_identifier: TokenIdentifier, amount: BigUint) -> SCResult<()> {
		only_owner!(self, "only owner may call this function");

		self.send().esdt_local_mint(
			self.get_gas_left(),
			token_identifier.as_esdt_identifier(),
			&amount,
		);

		Ok(())
	}

	#[endpoint(localBurn)]
	fn local_burn(&self, token_identifier: TokenIdentifier, amount: BigUint) -> SCResult<()> {
		only_owner!(self, "only owner may call this function");

		self.send().esdt_local_burn(
			self.get_gas_left(),
			token_identifier.as_esdt_identifier(),
			&amount,
		);

		Ok(())
	}

	#[endpoint(setLocalRoles)]
	fn set_local_roles(
		&self,
		address: Address,
		token_identifier: TokenIdentifier,
		#[var_args] roles: VarArgs<EsdtLocalRole>,
	) -> SCResult<AsyncCall<BigUint>> {
		only_owner!(self, "only owner may call this function");

		Ok(ESDTSystemSmartContractProxy::new()
			.set_special_roles(
				&address,
				token_identifier.as_esdt_identifier(),
				roles.as_slice(),
			)
			.async_call()
			.with_callback(self.callbacks().change_roles_callback()))
	}

	#[endpoint(unsetLocalRoles)]
	fn unset_local_roles(
		&self,
		address: Address,
		token_identifier: TokenIdentifier,
		#[var_args] roles: VarArgs<EsdtLocalRole>,
	) -> SCResult<AsyncCall<BigUint>> {
		only_owner!(self, "only owner may call this function");

		Ok(ESDTSystemSmartContractProxy::new()
			.unset_special_roles(
				&address,
				token_identifier.as_esdt_identifier(),
				roles.as_slice(),
			)
			.async_call()
			.with_callback(self.callbacks().change_roles_callback()))
	}

	// views

	#[view(getFungibleEsdtBalance)]
	fn get_fungible_esdt_balance(&self, token_identifier: &TokenIdentifier) -> BigUint {
		self.get_esdt_balance(
			&self.get_sc_address(),
			token_identifier.as_esdt_identifier(),
			0,
		)
	}

	// callbacks

	#[callback]
	fn esdt_issue_callback(
		&self,
		caller: &Address,
		#[payment_token] token_identifier: TokenIdentifier,
		#[payment] returned_tokens: BigUint,
		#[call_result] result: AsyncCallResult<()>,
	) {
		// callback is called with ESDTTransfer of the newly issued token, with the amount requested,
		// so we can get the token identifier and amount from the call data
		match result {
			AsyncCallResult::Ok(()) => {
				self.last_issued_token().set(&token_identifier);
				self.last_error_message().clear();
			},
			AsyncCallResult::Err(message) => {
				// return issue cost to the owner
				if token_identifier.is_egld() && returned_tokens > 0 {
					self.send().direct_egld(caller, &returned_tokens, &[]);
				}

				self.last_error_message().set(&message.err_msg);
			},
		}
	}

	#[callback]
	fn change_roles_callback(&self, #[call_result] result: AsyncCallResult<()>) {
		match result {
			AsyncCallResult::Ok(()) => {
				self.last_error_message().clear();
			},
			AsyncCallResult::Err(message) => {
				self.last_error_message().set(&message.err_msg);
			},
		}
	}

	// storage

	#[view(lastIssuedToken)]
	#[storage_mapper("lastIssuedToken")]
	fn last_issued_token(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

	#[view(lastErrorMessage)]
	#[storage_mapper("lastErrorMessage")]
	fn last_error_message(&self) -> SingleValueMapper<Self::Storage, BoxedBytes>;
}
