#![no_std]

elrond_wasm::imports!();

/// Test contract for investigating async calls.
#[elrond_wasm_derive::contract]
pub trait RecursiveCaller {
	#[proxy]
	fn vault_proxy(&self, to: Address) -> vault::Proxy<Self::SendApi>;

	#[proxy]
	fn self_proxy(&self, to: Address) -> self::Proxy<Self::SendApi>;

	#[init]
	fn init(&self) {}

	#[endpoint]
	fn recursive_send_funds(
		&self,
		to: &Address,
		token_identifier: &TokenIdentifier,
		amount: &Self::BigUint,
		counter: u32,
	) -> AsyncCall<Self::SendApi> {
		self.recursive_send_funds_event(to, token_identifier, amount, counter);

		self.vault_proxy(to.clone())
			.accept_funds(token_identifier.clone(), amount.clone())
			.async_call()
			.with_callback(self.callbacks().recursive_send_funds_callback(
				to,
				token_identifier,
				amount,
				counter,
			))
	}

	#[callback]
	fn recursive_send_funds_callback(
		&self,
		to: &Address,
		token_identifier: &TokenIdentifier,
		amount: &Self::BigUint,
		counter: u32,
	) -> OptionalResult<AsyncCall<Self::SendApi>> {
		self.recursive_send_funds_callback_event(to, token_identifier, amount, counter);

		if counter > 1 {
			OptionalResult::Some(
				self.self_proxy(self.blockchain().get_sc_address())
					.recursive_send_funds(to, token_identifier, amount, counter - 1)
					.async_call(),
			)
		} else {
			OptionalResult::None
		}
	}

	#[event("recursive_send_funds")]
	fn recursive_send_funds_event(
		&self,
		#[indexed] to: &Address,
		#[indexed] token_identifier: &TokenIdentifier,
		#[indexed] amount: &Self::BigUint,
		counter: u32,
	);

	#[event("recursive_send_funds_callback")]
	fn recursive_send_funds_callback_event(
		&self,
		#[indexed] to: &Address,
		#[indexed] token_identifier: &TokenIdentifier,
		#[indexed] amount: &Self::BigUint,
		counter: u32,
	);
}
