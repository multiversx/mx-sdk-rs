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
	) {
		self.send().direct(&to, &token, &payment, &[]);
	}

	#[endpoint]
	#[payable("*")]
	fn forward_call(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: BigUint,
		endpoint_name: BoxedBytes,
		#[var_args] args: VarArgs<BoxedBytes>,
	) {
		self.send()
			.async_call(&to, &token, &payment, endpoint_name.as_slice(), args);
	}

	#[endpoint]
	#[payable("*")]
	fn forward_call_half_payment(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: BigUint,
		endpoint_name: BoxedBytes,
		#[var_args] args: VarArgs<BoxedBytes>,
	) {
		let half_payment = payment / 2u32.into();
		self.send()
			.async_call(&to, &token, &half_payment, endpoint_name.as_slice(), args);
	}

	#[view]
	#[storage_mapper("callback_raw_data")]
	fn callback_raw_data(
		&self,
	) -> VecMapper<Self::Storage, (TokenIdentifier, BigUint, Vec<BoxedBytes>)>;

	#[view]
	fn callback_data_at_index(
		&self,
		index: usize,
	) -> MultiResult3<TokenIdentifier, BigUint, MultiResultVec<BoxedBytes>> {
		let (token, payment, args) = self.callback_raw_data().get(index);
		(token, payment, args.into()).into()
	}

	#[endpoint]
	fn clear_callback_info(&self) {
		self.callback_raw_data().clear();
	}

	#[callback_raw]
	fn callback_raw(
		&self,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: BigUint,
		#[var_args] args: VarArgs<BoxedBytes>,
	) {
		self.callback_raw_data()
			.push(&(token, payment, args.into_vec()));
	}
}
