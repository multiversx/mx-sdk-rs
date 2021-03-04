#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const ESDT_ISSUE_COST: u64 = 5000000000000000000; // 5 EGLD
const EGLD_DECIMALS: u8 = 18;

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
		if result.is_ok() {
			self.unused_wrapped_egld().set(&initial_supply);
			self.wrapped_egld_token_id().set(&token_identifier);
		}
		// nothing to do in case of error
	}

	#[endpoint(mintWrappedEgld)]
	fn mint_wrapped_egld(&self, amount: BigUint) -> SCResult<AsyncCall<BigUint>> {
		only_owner!(self, "only owner may call this function");

		require!(
			!self.wrapped_egld_token_id().is_empty(),
			"Wrapped EGLD was not issued yet"
		);

		Ok(ESDTSystemSmartContractProxy::new()
			.mint(
				self.wrapped_egld_token_id().get().as_esdt_name(),
				&amount,
			)
			.async_call()
			.with_callback(self.callbacks().esdt_mint_callback(&amount)))
	}

	#[callback]
	fn esdt_mint_callback(&self, amount: &BigUint, #[call_result] result: AsyncCallResult<()>) {
		if result.is_ok() {
			self.unused_wrapped_egld()
				.update(|unused_wrapped_egld| *unused_wrapped_egld += amount);
		}
		// nothing to do in case of error
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

		self.send().direct_esdt_via_transf_exec(
			&self.get_caller(),
			self.wrapped_egld_token_id().get().as_slice(),
			&payment,
			b"wrapping",
		);

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
		self.send()
			.direct_egld(&self.get_caller(), &wrapped_egld_payment, b"unwrapping");

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
}
