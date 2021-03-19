#![no_std]

elrond_wasm::imports!();

#[elrond_wasm_derive::callable(VaultProxy)]
pub trait Vault {
	fn echo_arguments(
		&self,
		args: VarArgs<BoxedBytes>,
	) -> ContractCall<BigUint, VarArgs<BoxedBytes>>;

	#[payable("*")]
	fn accept_funds(&self) -> ContractCall<BigUint, ()>;

	#[payable("*")]
	fn reject_funds(&self) -> ContractCall<BigUint, ()>;

	fn retrieve_funds(&self, token: TokenIdentifier, amount: BigUint) -> ContractCall<BigUint, ()>;
}

/// Test contract for investigating async calls.
#[elrond_wasm_derive::contract(ForwarderImpl)]
pub trait Forwarder {
	#[init]
	fn init(&self) {}

	#[endpoint]
	fn send_egld(
		&self,
		to: &Address,
		amount: &BigUint,
		#[var_args] opt_data: OptionalArg<BoxedBytes>,
	) {
		let data = match &opt_data {
			OptionalArg::Some(data) => data.as_slice(),
			OptionalArg::None => &[],
		};
		self.send().direct_egld(to, amount, data);
	}

	#[endpoint]
	fn send_esdt(
		&self,
		to: &Address,
		token_id: BoxedBytes,
		amount: &BigUint,
		#[var_args] opt_data: OptionalArg<BoxedBytes>,
	) {
		let data = match &opt_data {
			OptionalArg::Some(data) => data.as_slice(),
			OptionalArg::None => &[],
		};
		self.send()
			.direct_esdt_via_transf_exec(to, token_id.as_slice(), amount, data);
	}

	#[endpoint]
	fn send_esdt_twice(
		&self,
		to: &Address,
		token_id: BoxedBytes,
		amount_first_time: &BigUint,
		amount_second_time: &BigUint,
		#[var_args] opt_data: OptionalArg<BoxedBytes>,
	) {
		let data = match &opt_data {
			OptionalArg::Some(data) => data.as_slice(),
			OptionalArg::None => &[],
		};
		self.send()
			.direct_esdt_via_transf_exec(to, token_id.as_slice(), amount_first_time, data);
		self.send()
			.direct_esdt_via_transf_exec(to, token_id.as_slice(), amount_second_time, data);
	}

	#[endpoint]
	#[payable("*")]
	fn forward_async_call(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: BigUint,
	) -> AsyncCall<BigUint> {
		contract_call!(self, to, VaultProxy)
			.with_token_transfer(token, payment)
			.accept_funds()
			.async_call()
	}

	#[endpoint]
	#[payable("*")]
	fn forward_async_call_half_payment(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: BigUint,
	) -> AsyncCall<BigUint> {
		let half_payment = payment / 2u32.into();
		contract_call!(self, to, VaultProxy)
			.with_token_transfer(token, half_payment)
			.accept_funds()
			.async_call()
	}

	#[endpoint]
	#[payable("*")]
	fn retrieve_funds(
		&self,
		to: Address,
		token: TokenIdentifier,
		payment: BigUint,
	) -> AsyncCall<BigUint> {
		contract_call!(self, to, VaultProxy)
			.retrieve_funds(token, payment)
			.async_call()
			.with_callback(self.callbacks().retrieve_funds_callback())
	}

	#[callback]
	fn retrieve_funds_callback(
		&self,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: BigUint,
	) {
		let _ = self.callback_data().push(&(
			BoxedBytes::from(&b"retrieve_funds_callback"[..]),
			token,
			payment,
			Vec::new(),
		));
	}

	#[endpoint]
	fn send_funds_twice(
		&self,
		to: &Address,
		token_identifier: &TokenIdentifier,
		amount: &BigUint,
	) -> AsyncCall<BigUint> {
		contract_call!(self, to.clone(), VaultProxy)
			.with_token_transfer(token_identifier.clone(), amount.clone())
			.accept_funds()
			.async_call()
			.with_callback(
				self.callbacks()
					.send_funds_twice_callback(to, token_identifier, amount),
			)
	}

	#[callback]
	fn send_funds_twice_callback(
		&self,
		to: &Address,
		token_identifier: &TokenIdentifier,
		amount: &BigUint,
	) -> AsyncCall<BigUint> {
		contract_call!(self, to.clone(), VaultProxy)
			.with_token_transfer(token_identifier.clone(), amount.clone())
			.accept_funds()
			.async_call()
	}

	#[view]
	#[storage_mapper("callback_data")]
	fn callback_data(
		&self,
	) -> VecMapper<Self::Storage, (BoxedBytes, TokenIdentifier, BigUint, Vec<BoxedBytes>)>;

	#[view]
	fn callback_data_at_index(
		&self,
		index: usize,
	) -> MultiResult4<BoxedBytes, TokenIdentifier, BigUint, MultiResultVec<BoxedBytes>> {
		let (cb_name, token, payment, args) = self.callback_data().get(index);
		(cb_name, token, payment, args.into()).into()
	}

	#[endpoint]
	fn clear_callback_data(&self) {
		self.callback_data().clear();
	}

	#[endpoint]
	#[payable("*")]
	fn echo_arguments_sync(&self, to: Address, #[var_args] args: VarArgs<BoxedBytes>) {
		let half_gas = self.get_gas_left() / 2;

		let result = contract_call!(self, to, VaultProxy)
			.echo_arguments(args)
			.execute_on_dest_context(half_gas, self.send());

		self.execute_on_dest_context_result(result.as_slice());
	}

	#[event("execute_on_dest_context_result")]
	fn execute_on_dest_context_result(&self, result: &[BoxedBytes]);
}
