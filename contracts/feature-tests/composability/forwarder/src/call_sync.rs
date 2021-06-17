elrond_wasm::imports!();

#[elrond_wasm_derive::module]
pub trait ForwarderSyncCallModule {
	#[proxy]
	fn vault_proxy(&self, to: Address) -> vault::Proxy<Self::SendApi>;

	#[endpoint]
	#[payable("*")]
	fn echo_arguments_sync(&self, to: Address, #[var_args] args: VarArgs<BoxedBytes>) {
		let half_gas = self.blockchain().get_gas_left() / 2;

		let result = self
			.vault_proxy(to)
			.echo_arguments(args)
			.with_gas_limit(half_gas)
			.execute_on_dest_context();

		self.execute_on_dest_context_result_event(result.as_slice());
	}

	#[endpoint]
	#[payable("*")]
	fn echo_arguments_sync_range(
		&self,
		to: Address,
		start: usize,
		end: usize,
		#[var_args] args: VarArgs<BoxedBytes>,
	) {
		let half_gas = self.blockchain().get_gas_left() / 2;

		let result = self
			.vault_proxy(to)
			.echo_arguments(args)
			.with_gas_limit(half_gas)
			.execute_on_dest_context_custom_range(|_, _| (start, end));

		self.execute_on_dest_context_result_event(result.as_slice());
	}

	#[endpoint]
	#[payable("*")]
	fn echo_arguments_sync_twice(&self, to: Address, #[var_args] args: VarArgs<BoxedBytes>) {
		let one_third_gas = self.blockchain().get_gas_left() / 3;

		let result = self
			.vault_proxy(to.clone())
			.echo_arguments(args.clone())
			.with_gas_limit(one_third_gas)
			.execute_on_dest_context();

		self.execute_on_dest_context_result_event(result.as_slice());

		let result = self
			.vault_proxy(to)
			.echo_arguments(args)
			.with_gas_limit(one_third_gas)
			.execute_on_dest_context();

		self.execute_on_dest_context_result_event(result.as_slice());
	}

	#[event("echo_arguments_sync_result")]
	fn execute_on_dest_context_result_event(&self, result: &[BoxedBytes]);

	#[endpoint]
	#[payable("*")]
	fn forward_sync_accept_funds(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment_amount] payment: Self::BigUint,
		#[payment_nonce] token_nonce: u64,
	) {
		let half_gas = self.blockchain().get_gas_left() / 2;

		let result: MultiResult4<TokenIdentifier, BoxedBytes, Self::BigUint, u64> = self
			.vault_proxy(to)
			.accept_funds_echo_payment(token, payment, token_nonce)
			.with_gas_limit(half_gas)
			.execute_on_dest_context();

		let (token_identifier, token_type_str, token_payment, token_nonce) = result.into_tuple();
		self.accept_funds_sync_result_event(
			&token_identifier,
			token_type_str.as_slice(),
			&token_payment,
			token_nonce,
		);
	}

	#[event("accept_funds_sync_result")]
	fn accept_funds_sync_result_event(
		&self,
		#[indexed] token_identifier: &TokenIdentifier,
		#[indexed] token_type: &[u8],
		#[indexed] token_payment: &Self::BigUint,
		#[indexed] token_nonce: u64,
	);

	#[endpoint]
	#[payable("*")]
	fn forward_sync_accept_funds_then_read(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment_amount] payment: Self::BigUint,
		#[payment_nonce] token_nonce: u64,
	) -> usize {
		let _ = self
			.vault_proxy(to.clone())
			.with_nft_nonce(token_nonce)
			.accept_funds(token, payment)
			.execute_on_dest_context();

		self.vault_proxy(to)
			.call_counts(b"accept_funds")
			.execute_on_dest_context()
	}

	#[endpoint]
	fn forward_sync_retrieve_funds(
		&self,
		to: Address,
		token: TokenIdentifier,
		token_nonce: u64,
		amount: Self::BigUint,
	) {
		self.vault_proxy(to)
			.retrieve_funds(token, token_nonce, amount, OptionalArg::None)
			.execute_on_dest_context()
	}
}
