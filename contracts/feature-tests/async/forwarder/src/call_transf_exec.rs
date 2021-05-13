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
		#[payment] payment: Self::BigUint,
	) {
		let token_nonce = self.call_value().esdt_token_nonce();
		self.vault_proxy(to)
			.accept_funds(token, payment)
			.with_nft_nonce(token_nonce)
			.transfer_execute(self.blockchain().get_gas_left());
	}

	#[endpoint]
	#[payable("*")]
	fn forward_transf_exec_accept_funds_twice(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: Self::BigUint,
	) {
		let token_nonce = self.call_value().esdt_token_nonce();
		let half_payment = payment / Self::BigUint::from(2u32);
		let half_gas = self.blockchain().get_gas_left() / 2;

		self.vault_proxy(to.clone())
			.accept_funds(token.clone(), half_payment.clone())
			.with_nft_nonce(token_nonce)
			.transfer_execute(half_gas);

		self.vault_proxy(to)
			.accept_funds(token, half_payment)
			.with_nft_nonce(token_nonce)
			.transfer_execute(half_gas);
	}
}
