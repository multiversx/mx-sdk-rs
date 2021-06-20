#![no_std]

elrond_wasm::imports!();

const EGLD_DECIMALS: usize = 18;

#[elrond_wasm_derive::contract]
pub trait Child {
	#[init]
	fn init(&self) {}

	#[payable("EGLD")]
	#[endpoint(issueWrappedEgld)]
	fn issue_wrapped_egld(
		&self,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
		initial_supply: Self::BigUint,
		#[payment] issue_cost: Self::BigUint,
	) -> AsyncCall<Self::SendApi> {
		ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.issue_fungible(
				issue_cost,
				&token_display_name,
				&token_ticker,
				&initial_supply,
				FungibleTokenProperties {
					num_decimals: EGLD_DECIMALS,
					can_freeze: false,
					can_wipe: false,
					can_pause: false,
					can_mint: true,
					can_burn: false,
					can_change_owner: false,
					can_upgrade: true,
					can_add_special_roles: true,
				},
			)
			.async_call()
			.with_callback(self.callbacks().esdt_issue_callback())
	}

	// callbacks

	#[callback]
	fn esdt_issue_callback(
		&self,
		#[payment_token] token_identifier: TokenIdentifier,
		#[payment] _amount: Self::BigUint,
		#[call_result] _result: AsyncCallResult<()>,
	) {
		self.wrapped_egld_token_identifier().set(&token_identifier);
	}

	// storage

	#[view(getWrappedEgldTokenIdentifier)]
	#[storage_mapper("wrappedEgldTokenIdentifier")]
	fn wrapped_egld_token_identifier(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;
}
