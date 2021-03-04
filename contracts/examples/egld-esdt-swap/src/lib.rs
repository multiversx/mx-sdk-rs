#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const ESDT_ISSUE_COST: u64 = 5000000000000000000; // 5 EGLD
const EGLD_DECIMALS: usize = 18;

/// Converts between EGLD and a wrapped EGLD ESDT token.
///	1 EGLD = 1 wrapped EGLD and is interchangeable at all times.
/// Also manages the supply of wrapped EGLD tokens.
#[elrond_wasm_derive::contract(EgldEsdtSwapImpl)]
pub trait EgldEsdtSwap {
	#[init]
	fn init(&self) {}

	// endpoints - owner-only

	#[payable("EGLD")]
	#[endpoint(issueWrappedEgld)]
	fn issue_wrapped_egld(
		&self,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
		initial_supply: BigUint,
		#[payment] payment: BigUint,
	) -> SCResult<AsyncCall<BigUint>> {
		only_owner!(self, "only owner may call this function");

		require!(
			self.wrapped_egld_token_id().is_empty(),
			"wrapped egld was already issued"
		);
		require!(
			payment == BigUint::from(ESDT_ISSUE_COST),
			"Wrong payment, should pay exactly 5 EGLD for ESDT token issue"
		);

		self.issue_started_event(token_ticker.as_slice(), &initial_supply);

		Ok(ESDTSystemSmartContractProxy::new()
			.issue(
				ESDT_ISSUE_COST.into(),
				&token_display_name,
				&token_ticker,
				&initial_supply,
				EGLD_DECIMALS,
				false,
				false,
				false,
				true,
				true,
				true,
				true,
			)
			.async_call()
			.with_callback(self.callbacks().esdt_issue_callback()))
	}

	#[callback]
	fn esdt_issue_callback(
		&self,
		#[payment_token] token_identifier: TokenIdentifier,
		#[payment] initial_supply: BigUint,
		#[call_result] result: AsyncCallResult<()>,
	) {
		// callback is called with ESDTTransfer of the newly issued token, with the amount requested,
		// so we can get the token identifier and amount from the call data
		match result {
			AsyncCallResult::Ok(()) => {
				self.issue_success_event(&token_identifier, &initial_supply);
				self.unused_wrapped_egld().set(&initial_supply);
				self.wrapped_egld_token_id().set(&token_identifier);
			},
			AsyncCallResult::Err(message) => {
				self.issue_failure_event(message.err_msg.as_slice());
			},
		}
	}

	#[endpoint(mintWrappedEgld)]
	fn mint_wrapped_egld(&self, amount: BigUint) -> SCResult<AsyncCall<BigUint>> {
		only_owner!(self, "only owner may call this function");

		require!(
			!self.wrapped_egld_token_id().is_empty(),
			"Wrapped EGLD was not issued yet"
		);

		let wrapped_egld_token_id = self.wrapped_egld_token_id().get();
		let esdt_token_id = wrapped_egld_token_id.as_esdt_name();
		self.mint_started_event(esdt_token_id, &amount);

		Ok(ESDTSystemSmartContractProxy::new()
			.mint(esdt_token_id, &amount)
			.async_call()
			.with_callback(self.callbacks().esdt_mint_callback(&amount)))
	}

	#[callback]
	fn esdt_mint_callback(&self, amount: &BigUint, #[call_result] result: AsyncCallResult<()>) {
		match result {
			AsyncCallResult::Ok(()) => {
				self.mint_success_event();
				self.unused_wrapped_egld()
					.update(|unused_wrapped_egld| *unused_wrapped_egld += amount);
			},
			AsyncCallResult::Err(message) => {
				self.mint_failure_event(message.err_msg.as_slice());
			},
		}
	}

	// endpoints

	#[payable("EGLD")]
	#[endpoint(wrapEgld)]
	fn wrap_egld(&self, #[payment] payment: BigUint) -> SCResult<()> {
		require!(payment > 0, "Payment must be more than 0");
		require!(
			!self.wrapped_egld_token_id().is_empty(),
			"Wrapped EGLD was not issued yet"
		);

		let mut unused_wrapped_egld = self.unused_wrapped_egld().get();
		require!(
			unused_wrapped_egld > payment,
			"Contract does not have enough wrapped EGLD. Please try again once more is minted."
		);
		unused_wrapped_egld -= &payment;
		self.unused_wrapped_egld().set(&unused_wrapped_egld);

		let caller = self.get_caller();
		self.send().direct_esdt_via_transf_exec(
			&caller,
			self.wrapped_egld_token_id().get().as_slice(),
			&payment,
			b"wrapping",
		);

		self.wrap_egld_event(&caller, &payment);

		Ok(())
	}

	#[payable("*")]
	#[endpoint(unwrapEgld)]
	fn unwrap_egld(
		&self,
		#[payment] wrapped_egld_payment: BigUint,
		#[payment_token] token_identifier: TokenIdentifier,
	) -> SCResult<()> {
		require!(
			!self.wrapped_egld_token_id().is_empty(),
			"Wrapped EGLD was not issued yet"
		);
		require!(token_identifier.is_esdt(), "Only ESDT tokens accepted");

		let wrapped_egld_token_identifier = self.wrapped_egld_token_id().get();

		require!(
			token_identifier == wrapped_egld_token_identifier,
			"Wrong esdt token"
		);

		require!(wrapped_egld_payment > 0, "Must pay more than 0 tokens!");
		// this should never happen, but we'll check anyway
		require!(
			wrapped_egld_payment <= self.get_sc_balance(),
			"Contract does not have enough funds"
		);

		self.unused_wrapped_egld()
			.update(|unused_wrapped_egld| *unused_wrapped_egld += &wrapped_egld_payment);

		// 1 wrapped EGLD = 1 EGLD, so we pay back the same amount
		let caller = self.get_caller();
		self.send()
			.direct_egld(&caller, &wrapped_egld_payment, b"unwrapping");

		self.unwrap_egld_event(&caller, &wrapped_egld_payment);

		Ok(())
	}

	#[view(getLockedEgldBalance)]
	fn get_locked_egld_balance() -> BigUint {
		self.get_sc_balance()
	}

	// storage

	#[view(getWrappedEgldTokenIdentifier)]
	#[storage_mapper("wrapped_egld_token_id")]
	fn wrapped_egld_token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

	#[view(getUnusedWrappedEgld)]
	#[storage_mapper("unused_wrapped_egld")]
	fn unused_wrapped_egld(&self) -> SingleValueMapper<Self::Storage, BigUint>;

	// events

	#[event("issue-started")]
	fn issue_started_event(&self, #[indexed] token_ticker: &[u8], initial_supply: &BigUint);

	#[event("issue-success")]
	fn issue_success_event(
		&self,
		#[indexed] token_identifier: &TokenIdentifier,
		initial_supply: &BigUint,
	);

	#[event("issue-failure")]
	fn issue_failure_event(&self, message: &[u8]);

	#[event("mint-started")]
	fn mint_started_event(&self, #[indexed] esdt_token_id: &[u8], amount: &BigUint);

	#[event("mint-success")]
	fn mint_success_event();

	#[event("mint-failure")]
	fn mint_failure_event(&self, message: &[u8]);

	#[event("wrap-egld")]
	fn wrap_egld_event(&self, #[indexed] user: &Address, amount: &BigUint);

	#[event("unwrap-egld")]
	fn unwrap_egld_event(&self, #[indexed] user: &Address, amount: &BigUint);
}
