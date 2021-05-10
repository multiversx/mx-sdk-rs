elrond_wasm::imports!();

#[elrond_wasm_derive::module]
pub trait ForwarderSyncCallModule {
	#[proxy]
	fn vault_proxy(&self, to: Address) -> vault::ProxyObj<Self::SendApi>;

	#[endpoint]
	#[payable("*")]
	fn echo_arguments_sync(&self, to: Address, #[var_args] args: VarArgs<BoxedBytes>) {
		let half_gas = self.blockchain().get_gas_left() / 2;

		let result = self
			.vault_proxy(to)
			.echo_arguments(args)
			.execute_on_dest_context(half_gas);

		self.execute_on_dest_context_result(result.as_slice());
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

		self.execute_on_dest_context_result(result.as_slice());
	}

	#[endpoint]
	#[payable("*")]
	fn echo_arguments_sync_twice(&self, to: Address, #[var_args] args: VarArgs<BoxedBytes>) {
		let one_third_gas = self.blockchain().get_gas_left() / 3;

		let result = self
			.vault_proxy(to.clone())
			.echo_arguments(args.clone())
			.execute_on_dest_context(one_third_gas);

		self.execute_on_dest_context_result(result.as_slice());

		let result = self
			.vault_proxy(to)
			.echo_arguments(args)
			.execute_on_dest_context(one_third_gas);

		self.execute_on_dest_context_result(result.as_slice());
	}

	#[event("execute_on_dest_context_result")]
	fn execute_on_dest_context_result(&self, result: &[BoxedBytes]);

	#[endpoint]
	#[payable("*")]
	fn forward_sync_accept_funds(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: Self::BigUint,
	) {
		let half_gas = self.blockchain().get_gas_left() / 2;

		let () = self
			.vault_proxy(to)
			.accept_funds(token, payment)
			.execute_on_dest_context(half_gas);
	}
}
