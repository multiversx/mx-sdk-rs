elrond_wasm::imports!();

type CallbackDataTuple<BigUint> = (BoxedBytes, TokenIdentifier, BigUint, Vec<BoxedBytes>);

#[elrond_wasm_derive::module]
pub trait ForwarderAsyncCallModule {
	#[proxy]
	fn vault_proxy(&self, to: Address) -> vault::Proxy<Self::SendApi>;

	#[endpoint]
	#[payable("*")]
	fn forward_async_accept_funds(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment_amount] payment: Self::BigUint,
		#[payment_nonce] token_nonce: u64,
	) -> AsyncCall<Self::SendApi> {
		self.vault_proxy(to)
			.accept_funds(token, payment)
			.with_nft_nonce(token_nonce)
			.async_call()
	}

	#[endpoint]
	#[payable("*")]
	fn forward_async_accept_funds_half_payment(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: Self::BigUint,
	) -> AsyncCall<Self::SendApi> {
		let half_payment = payment / 2u32.into();
		self.vault_proxy(to)
			.accept_funds(token, half_payment)
			.async_call()
	}

	#[endpoint]
	#[payable("*")]
	fn retrieve_funds(
		&self,
		to: Address,
		token: TokenIdentifier,
		payment: Self::BigUint,
	) -> AsyncCall<Self::SendApi> {
		self.vault_proxy(to)
			.retrieve_funds(token, payment, OptionalArg::None)
			.async_call()
			.with_callback(self.callbacks().retrieve_funds_callback())
	}

	#[callback]
	fn retrieve_funds_callback(
		&self,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: Self::BigUint,
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
		amount: &Self::BigUint,
	) -> AsyncCall<Self::SendApi> {
		self.vault_proxy(to.clone())
			.accept_funds(token_identifier.clone(), amount.clone())
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
		cb_amount: &Self::BigUint,
	) -> AsyncCall<Self::SendApi> {
		self.vault_proxy(to.clone())
			.accept_funds(token_identifier.clone(), cb_amount.clone())
			.async_call()
	}

	#[view]
	#[storage_mapper("callback_data")]
	fn callback_data(&self) -> VecMapper<Self::Storage, CallbackDataTuple<Self::BigUint>>;

	#[view]
	fn callback_data_at_index(
		&self,
		index: usize,
	) -> MultiResult4<BoxedBytes, TokenIdentifier, Self::BigUint, MultiResultVec<BoxedBytes>> {
		let (cb_name, token, payment, args) = self.callback_data().get(index);
		(cb_name, token, payment, args.into()).into()
	}

	#[endpoint]
	fn clear_callback_data(&self) {
		self.callback_data().clear();
	}
}
