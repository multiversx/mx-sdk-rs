#![no_std]

elrond_wasm::imports!();

#[elrond_wasm_derive::callable(VaultProxy)]
pub trait Vault {
	fn echo_arguments(&self, args: VarArgs<BoxedBytes>) -> ContractCall<BigUint>;

	#[payable("*")]
	fn accept_funds(&self) -> ContractCall<BigUint>;

	#[payable("*")]
	fn reject_funds(&self) -> ContractCall<BigUint>;

	fn retrieve_funds(&self, token: TokenIdentifier, amount: BigUint) -> ContractCall<BigUint>;
}

#[elrond_wasm_derive::callable(AlsoRecursiveCallerProxy)]
pub trait AlsoRecursiveCaller {
	fn recursive_send_funds(
		&self,
		to: &Address,
		amount: &BigUint,
		counter: u32,
	) -> ContractCall<BigUint>;
}

/// Test contract for investigating async calls.
#[elrond_wasm_derive::contract(ForwarderImpl)]
pub trait RecursiveCaller {
	#[init]
	fn init(&self) {}

	#[endpoint]
	fn recursive_send_funds(
		&self,
		to: &Address,
		amount: &BigUint,
		counter: u32,
	) -> OptionalResult<AsyncCall<BigUint>> {
		self.recursive_send_funds_event(to, amount, counter);
		if counter > 0 {
			OptionalResult::Some(
				contract_call!(self, to.clone(), VaultProxy)
					.accept_funds()
					.async_call()
					.with_callback(
						self.callbacks()
							.recursive_send_funds_callback(to, amount, counter),
					),
			)
		} else {
			OptionalResult::None
		}
	}

	#[callback]
	fn recursive_send_funds_callback(
		&self,
		to: &Address,
		amount: &BigUint,
		counter: u32,
	) -> AsyncCall<BigUint> {
		self.recursive_send_funds_callback_event(to, amount, counter);

		contract_call!(self, to.clone(), AlsoRecursiveCallerProxy)
			.recursive_send_funds(&to, amount, counter - 1)
			.async_call()
	}

	#[event("recursive_send_funds")]
	fn recursive_send_funds_event(
		&self,
		#[indexed] to: &Address,
		#[indexed] amount: &BigUint,
		counter: u32,
	);

	#[event("recursive_send_funds")]
	fn recursive_send_funds_callback_event(
		&self,
		#[indexed] to: &Address,
		#[indexed] amount: &BigUint,
		counter: u32,
	);
}
