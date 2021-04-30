elrond_wasm::imports!();

use super::vault_proxy::*;

#[elrond_wasm_derive::module(ForwarderSyncCallModuleImpl)]
pub trait ForwarderSyncCallModule {
	#[endpoint]
	#[payable("*")]
	fn echo_arguments_sync(&self, to: Address, #[var_args] args: VarArgs<BoxedBytes>) {
		let half_gas = self.blockchain().get_gas_left() / 2;

		let result = contract_call!(self, to, VaultProxy)
			.echo_arguments(&args)
			.execute_on_dest_context(half_gas, self.send());

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

		let result = contract_call!(self, to, VaultProxy)
			.echo_arguments(&args)
			.execute_on_dest_context_custom_range(half_gas, |_, _| (start, end), self.send());

		self.execute_on_dest_context_result(result.as_slice());
	}

	#[endpoint]
	#[payable("*")]
	fn echo_arguments_sync_twice(&self, to: Address, #[var_args] args: VarArgs<BoxedBytes>) {
		let one_third_gas = self.blockchain().get_gas_left() / 3;

		let result = contract_call!(self, to.clone(), VaultProxy)
			.echo_arguments(&args)
			.execute_on_dest_context(one_third_gas, self.send());

		self.execute_on_dest_context_result(result.as_slice());

		let result = contract_call!(self, to, VaultProxy)
			.echo_arguments(&args)
			.execute_on_dest_context(one_third_gas, self.send());

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
		#[payment] payment: BigUint,
	) {
		let half_gas = self.blockchain().get_gas_left() / 2;

		let () = contract_call!(self, to, VaultProxy)
			.with_token_transfer(token, payment)
			.accept_funds()
			.execute_on_dest_context(half_gas, self.send());
	}
}
