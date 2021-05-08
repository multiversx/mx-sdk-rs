elrond_wasm::imports!();

use vault::Proxy as _;

#[elrond_wasm_derive::module(ForwarderAsyncCallModuleImpl)]
pub trait ForwarderAsyncCallModule {
	#[endpoint]
	#[payable("*")]
	fn forward_async_accept_funds(
		&self,
		to: Address,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: Self::BigUint,
	) -> AsyncCall<Self::SendApi> {
		vault::ProxyObj::new_proxy_obj(self.send(), to)
			.accept_funds(token, payment)
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
		vault::ProxyObj::new_proxy_obj(self.send(), to)
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
		vault::ProxyObj::new_proxy_obj(self.send(), to)
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
		vault::ProxyObj::new_proxy_obj(self.send(), to.clone())
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
		vault::ProxyObj::new_proxy_obj(self.send(), to.clone())
			.accept_funds(token_identifier.clone(), cb_amount.clone())
			.async_call()
	}

	#[view]
	#[storage_mapper("callback_data")]
	fn callback_data(
		&self,
	) -> VecMapper<Self::Storage, (BoxedBytes, TokenIdentifier, Self::BigUint, Vec<BoxedBytes>)>;

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
