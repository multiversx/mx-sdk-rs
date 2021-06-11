elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
pub enum ProgrammedCallType {
	SyncCall,
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct ProgrammedCall<BigUint: BigUintApi> {
	to: Address,
	payment_token: TokenIdentifier,
	payment_nonce: u64,
	payment_amount: BigUint,
}

#[elrond_wasm_derive::module]
pub trait ForwarderProgrammedCallModule {
	#[proxy]
	fn self_proxy(&self, to: Address) -> crate::Proxy<Self::SendApi>;

	#[view]
	#[storage_mapper("programmed_calls")]
	fn programmed_calls(&self) -> LinkedListMapper<Self::Storage, ProgrammedCall<Self::BigUint>>;

	#[endpoint]
	fn add_programmed_call(
		&self,
		to: Address,
		payment_token: TokenIdentifier,
		payment_nonce: u64,
		payment_amount: Self::BigUint,
	) {
		self.programmed_calls().push_back(ProgrammedCall {
			to,
			payment_token,
			payment_nonce,
			payment_amount,
		});
	}

	#[endpoint]
	fn forward_programmed_calls(&self) {
		while let Some(call) = self.programmed_calls().pop_front() {
			let () = self
				.self_proxy(call.to)
				.forward_programmed_calls()
				.execute_on_dest_context();

			// self.execute_on_dest_context_result_event(result.as_slice());
		}
	}

	#[event("forward_programmed_calls")]
	fn forward_programmed_calls_event(
		&self,
		#[indexed] token_identifier: &TokenIdentifier,
		#[indexed] token_type: &[u8],
		#[indexed] token_payment: &Self::BigUint,
		#[indexed] token_nonce: u64,
	);
}
