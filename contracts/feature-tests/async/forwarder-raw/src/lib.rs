#![no_std]

elrond_wasm::imports!();

/// Test contract for investigating async calls.
#[elrond_wasm_derive::contract(ForwarderRawImpl)]
pub trait ForwarderRaw {
	#[init]
	fn init(&self) {}

	#[endpoint]
	#[payable("*")]
	fn forward_payment(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: BigUint,
	) -> SendToken<BigUint> {
		SendToken {
			to,
			token,
			amount: payment,
			data: BoxedBytes::empty(),
		}
	}

	#[endpoint]
	#[payable("*")]
	fn forward_direct_esdt_via_transf_exec(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: BigUint,
	) {
		self.send()
			.direct_esdt_via_transf_exec(&to, &token.as_slice(), &payment, &[]);
	}

	fn forward_contract_call(
		&self,
		to: Address,
		token: TokenIdentifier,
		payment: BigUint,
		endpoint_name: BoxedBytes,
		args: VarArgs<BoxedBytes>,
	) -> ContractCall<BigUint> {
		let mut contract_call = ContractCall::new(to, token, payment, endpoint_name);
		for arg in args.into_vec() {
			contract_call.push_argument_raw_bytes(arg.as_slice());
		}
		contract_call
	}

	#[endpoint]
	#[payable("*")]
	fn forward_async_call(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: BigUint,
		endpoint_name: BoxedBytes,
		#[var_args] args: VarArgs<BoxedBytes>,
	) -> AsyncCall<BigUint> {
		self.forward_contract_call(to, token, payment, endpoint_name, args)
			.async_call()
	}

	#[endpoint]
	#[payable("*")]
	fn forward_async_call_half_payment(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: BigUint,
		endpoint_name: BoxedBytes,
		#[var_args] args: VarArgs<BoxedBytes>,
	) -> AsyncCall<BigUint> {
		let half_payment = payment / 2u32.into();
		self.forward_async_call(to, token, half_payment, endpoint_name, args)
	}

	#[endpoint]
	#[payable("EGLD")]
	fn forward_transf_exec_egld(
		&self,
		to: Address,
		#[payment] payment: BigUint,
		endpoint_name: BoxedBytes,
		#[var_args] args: VarArgs<BoxedBytes>,
	) -> TransferEgldExecute<BigUint> {
		self.forward_contract_call(to, TokenIdentifier::egld(), payment, endpoint_name, args)
			.transfer_egld_execute()
			.with_gas_limit(self.get_gas_left() / 2)
	}

	#[endpoint]
	#[payable("*")]
	fn forward_transf_exec_esdt(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: BigUint,
		endpoint_name: BoxedBytes,
		#[var_args] args: VarArgs<BoxedBytes>,
	) -> TransferEsdtExecute<BigUint> {
		self.forward_contract_call(to, token, payment, endpoint_name, args)
			.transfer_esdt_execute()
			.with_gas_limit(self.get_gas_left() / 2)
	}

	#[endpoint]
	#[payable("*")]
	fn forward_transf_exec(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: BigUint,
		endpoint_name: BoxedBytes,
		#[var_args] args: VarArgs<BoxedBytes>,
	) -> TransferExecute<BigUint> {
		self.forward_contract_call(to, token, payment, endpoint_name, args)
			.transfer_execute()
			.with_gas_limit(self.get_gas_left() / 2)
	}

	#[view]
	#[storage_mapper("callback_data")]
	fn callback_data(
		&self,
	) -> VecMapper<Self::Storage, (TokenIdentifier, BigUint, Vec<BoxedBytes>)>;

	#[view]
	fn callback_data_at_index(
		&self,
		index: usize,
	) -> MultiResult3<TokenIdentifier, BigUint, MultiResultVec<BoxedBytes>> {
		let (token, payment, args) = self.callback_data().get(index);
		(token, payment, args.into()).into()
	}

	#[endpoint]
	fn clear_callback_info(&self) {
		self.callback_data().clear();
	}

	#[callback_raw]
	fn callback_raw(
		&self,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: BigUint,
		#[var_args] args: VarArgs<BoxedBytes>,
	) {
		let args_vec = args.into_vec();
		self.callback_raw_event(&token, &payment, args_vec.as_slice());

		self.callback_data()
			.push(&(token, payment, args_vec));
	}

	#[event("callback_raw")]
	fn callback_raw_event(&self, #[indexed] token: &TokenIdentifier, #[indexed] payment: &BigUint, arguments: &[BoxedBytes]);
}
