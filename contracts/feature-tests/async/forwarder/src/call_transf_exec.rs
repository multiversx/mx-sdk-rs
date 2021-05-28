elrond_wasm::imports!();

#[elrond_wasm_derive::module]
pub trait ForwarderTransferExecuteModule {
	#[proxy]
	fn vault_proxy(&self, to: Address) -> vault::Proxy<Self::SendApi>;

	#[endpoint]
	#[payable("*")]
	fn forward_transf_exec_accept_funds(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment_amount] payment: Self::BigUint,
		#[payment_nonce] token_nonce: u64,
	) {
		self.vault_proxy(to)
			.accept_funds(token, payment)
			.with_nft_nonce(token_nonce)
			.transfer_execute();
	}

	#[endpoint]
	#[payable("*")]
	fn forward_transf_exec_accept_funds_twice(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment_amount] payment: Self::BigUint,
		#[payment_nonce] token_nonce: u64,
	) {
		let half_payment = payment / Self::BigUint::from(2u32);
		let half_gas = self.blockchain().get_gas_left() / 2;

		self.vault_proxy(to.clone())
			.accept_funds(token.clone(), half_payment.clone())
			.with_nft_nonce(token_nonce)
			.with_gas_limit(half_gas)
			.transfer_execute();

		self.vault_proxy(to)
			.accept_funds(token, half_payment)
			.with_nft_nonce(token_nonce)
			.with_gas_limit(half_gas)
			.transfer_execute();
	}

	/// Test that the default gas provided to the transfer_execute call
	/// leaves enough in the transaction for finish to happen.
	#[endpoint]
	#[payable("*")]
	fn forward_transf_exec_accept_funds_return_values(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment_amount] payment: Self::BigUint,
		#[payment_nonce] token_nonce: u64,
	) -> MultiResult4<u64, u64, Self::BigUint, TokenIdentifier> {
		let gas_left_before = self.blockchain().get_gas_left();

		self.vault_proxy(to)
			.accept_funds(token.clone(), payment)
			.with_nft_nonce(token_nonce)
			.transfer_execute();

		let gas_left_after = self.blockchain().get_gas_left();

		(gas_left_before, gas_left_after, Self::BigUint::zero(), token).into()
	}
}
