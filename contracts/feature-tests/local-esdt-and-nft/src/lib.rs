#![no_std]
#![allow(clippy::too_many_arguments)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

// used as mock attributes for NFTs
#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct Color {
	r: u8,
	g: u8,
	b: u8,
}

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
	) -> AsyncCall<BigUint> {
		let caller = self.get_caller();

		ESDTSystemSmartContractProxy::new()
			.issue_fungible(
				issue_cost,
				&token_display_name,
				&token_ticker,
				&initial_supply,
				FungibleTokenProperties {
					num_decimals: 0,
					can_freeze: true,
					can_wipe: true,
					can_pause: true,
					can_mint: true,
					can_burn: true,
					can_change_owner: true,
					can_upgrade: true,
					can_add_special_roles: true,
				},
			)
			.async_call()
			.with_callback(self.callbacks().esdt_issue_callback(&caller))
	}

	#[endpoint(localMint)]
	fn local_mint(&self, token_identifier: TokenIdentifier, amount: BigUint) {
		self.send().esdt_local_mint(
			self.get_gas_left(),
			token_identifier.as_esdt_identifier(),
			&amount,
		);
	}

	#[endpoint(localBurn)]
	fn local_burn(&self, token_identifier: TokenIdentifier, amount: BigUint) {
		self.send().esdt_local_burn(
			self.get_gas_left(),
			token_identifier.as_esdt_identifier(),
			&amount,
		);
	}

	// Non-Fungible Tokens

	#[payable("EGLD")]
	#[endpoint(nftIssue)]
	fn nft_issue(
		&self,
		#[payment] issue_cost: BigUint,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
	) -> AsyncCall<BigUint> {
		let caller = self.get_caller();

		ESDTSystemSmartContractProxy::new()
			.issue_non_fungible(
				issue_cost,
				&token_display_name,
				&token_ticker,
				NonFungibleTokenProperties {
					can_freeze: true,
					can_wipe: true,
					can_pause: true,
					can_change_owner: true,
					can_upgrade: true,
					can_add_special_roles: true,
				},
			)
			.async_call()
			.with_callback(self.callbacks().nft_issue_callback(&caller))
	}

	#[endpoint(nftCreate)]
	fn nft_create(
		&self,
		token_identifier: TokenIdentifier,
		amount: BigUint,
		name: BoxedBytes,
		royalties: BigUint,
		hash: H256,
		color: Color,
		uri: BoxedBytes,
	) {
		self.send().esdt_nft_create::<Color>(
			self.get_gas_left(),
			token_identifier.as_esdt_identifier(),
			&amount,
			&name,
			&royalties,
			&hash,
			&color,
			&[uri],
		);
	}

	#[endpoint(nftAddQuantity)]
	fn nft_add_quantity(&self, token_identifier: TokenIdentifier, nonce: u64, amount: BigUint) {
		self.send().esdt_nft_add_quantity(
			self.get_gas_left(),
			token_identifier.as_esdt_identifier(),
			nonce,
			&amount,
		);
	}

	#[endpoint(nftBurn)]
	fn nft_burn(&self, token_identifier: TokenIdentifier, nonce: u64, amount: BigUint) {
		self.send().esdt_nft_burn(
			self.get_gas_left(),
			token_identifier.as_esdt_identifier(),
			nonce,
			&amount,
		);
	}

	#[endpoint(transferNftViaAsyncCall)]
	fn transfer_nft_via_async_call(
		&self,
		to: Address,
		token_identifier: TokenIdentifier,
		nonce: u64,
		amount: BigUint,
		data: BoxedBytes,
	) {
		self.send().direct_esdt_nft_via_async_call(
			&self.get_sc_address(),
			&to,
			token_identifier.as_esdt_identifier(),
			nonce,
			&amount,
			data.as_slice(),
		);
	}

	#[endpoint]
	fn transfer_nft_and_execute(
		&self,
		to: Address,
		token_identifier: TokenIdentifier,
		nonce: u64,
		amount: BigUint,
		function: BoxedBytes,
		#[var_args] arguments: VarArgs<BoxedBytes>,
	) {
		let mut arg_buffer = ArgBuffer::new();
		for arg in arguments.into_vec() {
			arg_buffer.push_argument_bytes(arg.as_slice());
		}

		self.send().direct_esdt_nft_execute(
			&to,
			token_identifier.as_esdt_identifier(),
			nonce,
			&amount,
			self.get_gas_left(),
			function.as_slice(),
			&arg_buffer,
		);
	}

	// Semi-Fungible

	#[payable("EGLD")]
	#[endpoint(sftIssue)]
	fn sft_issue(
		&self,
		#[payment] issue_cost: BigUint,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
	) -> AsyncCall<BigUint> {
		let caller = self.get_caller();

		ESDTSystemSmartContractProxy::new()
			.issue_semi_fungible(
				issue_cost,
				&token_display_name,
				&token_ticker,
				SemiFungibleTokenProperties {
					can_freeze: true,
					can_wipe: true,
					can_pause: true,
					can_change_owner: true,
					can_upgrade: true,
					can_add_special_roles: true,
				},
			)
			.async_call()
			.with_callback(self.callbacks().nft_issue_callback(&caller))
	}

	// common

	#[endpoint(setLocalRoles)]
	fn set_local_roles(
		&self,
		address: Address,
		token_identifier: TokenIdentifier,
		#[var_args] roles: VarArgs<EsdtLocalRole>,
	) -> AsyncCall<BigUint> {
		ESDTSystemSmartContractProxy::new()
			.set_special_roles(
				&address,
				token_identifier.as_esdt_identifier(),
				roles.as_slice(),
			)
			.async_call()
			.with_callback(self.callbacks().change_roles_callback())
	}

	#[endpoint(unsetLocalRoles)]
	fn unset_local_roles(
		&self,
		address: Address,
		token_identifier: TokenIdentifier,
		#[var_args] roles: VarArgs<EsdtLocalRole>,
	) -> AsyncCall<BigUint> {
		ESDTSystemSmartContractProxy::new()
			.unset_special_roles(
				&address,
				token_identifier.as_esdt_identifier(),
				roles.as_slice(),
			)
			.async_call()
			.with_callback(self.callbacks().change_roles_callback())
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

	#[view(getNftBalance)]
	fn get_nft_balance(&self, token_identifier: &TokenIdentifier, nonce: u64) -> BigUint {
		self.get_esdt_balance(
			&self.get_sc_address(),
			token_identifier.as_esdt_identifier(),
			nonce,
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
				// return issue cost to the caller
				if token_identifier.is_egld() && returned_tokens > 0 {
					self.send().direct_egld(caller, &returned_tokens, &[]);
				}

				self.last_error_message().set(&message.err_msg);
			},
		}
	}

	#[callback]
	fn nft_issue_callback(
		&self,
		caller: &Address,
		#[call_result] result: AsyncCallResult<TokenIdentifier>,
	) {
		match result {
			AsyncCallResult::Ok(token_identifier) => {
				self.last_issued_token().set(&token_identifier);
				self.last_error_message().clear();
			},
			AsyncCallResult::Err(message) => {
				// return issue cost to the caller
				let (returned_tokens, token_identifier) = self.call_value().payment_token_pair();
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
