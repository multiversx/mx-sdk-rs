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
			.execute_on_dest_context(half_gas);

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
			.execute_on_dest_context_custom_range(half_gas, |_, _| (start, end));

		self.execute_on_dest_context_result_event(result.as_slice());
	}

	#[endpoint]
	#[payable("*")]
	fn echo_arguments_sync_twice(&self, to: Address, #[var_args] args: VarArgs<BoxedBytes>) {
		let one_third_gas = self.blockchain().get_gas_left() / 3;

		let result = self
			.vault_proxy(to.clone())
			.echo_arguments(args.clone())
			.execute_on_dest_context(one_third_gas);

		self.execute_on_dest_context_result_event(result.as_slice());

		let result = self
			.vault_proxy(to)
			.echo_arguments(args)
			.execute_on_dest_context(one_third_gas);

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
		#[payment] payment: Self::BigUint,
	) {
		let half_gas = self.blockchain().get_gas_left() / 2;

		let result: MultiResult4<TokenIdentifier, BoxedBytes, Self::BigUint, u64> = self
			.vault_proxy(to)
			.accept_funds_echo_payment(token, payment)
			.execute_on_dest_context(half_gas);
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
}
