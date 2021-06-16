elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm_derive::module]
pub trait EventsModule {
	#[event("issue-started")]
	fn issue_started_event(
		&self,
		#[indexed] caller: &Address,
		#[indexed] token_ticker: &[u8],
		initial_supply: &Self::BigUint,
	);

	#[event("issue-success")]
	fn issue_success_event(
		&self,
		#[indexed] caller: &Address,
		#[indexed] token_identifier: &TokenIdentifier,
		initial_supply: &Self::BigUint,
	);

	#[event("issue-failure")]
	fn issue_failure_event(&self, #[indexed] caller: &Address, message: &[u8]);

	#[view(getIssuedToken)]
	#[storage_mapper("issued_token")]
	fn issued_token(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

	#[event("mint-started")]
	fn mint_started_event(&self, #[indexed] caller: &Address, amount: &Self::BigUint);

	#[event("mint-success")]
	fn mint_success_event(&self, #[indexed] caller: &Address);

	#[event("mint-failure")]
	fn mint_failure_event(&self, #[indexed] caller: &Address, message: &[u8]);
	#[event("buy-token")]
	fn buy_token_event(&self, #[indexed] user: &Address, amount: &Self::BigUint);

	#[event("sell-token")]
	fn sell_token_event(&self, #[indexed] user: &Address, amount: &Self::BigUint);
}
