elrond_wasm::imports!();

use super::storage::*;

#[elrond_wasm_derive::module(ForwarderSftModuleImpl)]
pub trait ForwarderSftModule {
	#[module(ForwarderStorageModuleImpl)]
	fn storage_module(&self) -> ForwarderStorageModuleImpl<T, BigInt, BigUint>;

	#[payable("EGLD")]
	#[endpoint]
	fn sft_issue(
		&self,
		#[payment] issue_cost: BigUint,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
	) -> AsyncCall<BigUint> {
		let caller = self.blockchain().get_caller();

		ESDTSystemSmartContractProxy::new()
			.issue_semi_fungible(
				issue_cost,
				&token_display_name,
				&token_ticker,
				SemiFungibleTokenProperties {
					can_freeze: true,
					can_wipe: true,
					can_pause: true,
					can_change_owner: true,
					can_upgrade: true,
					can_add_special_roles: true,
				},
			)
			.async_call()
			.with_callback(self.callbacks().sft_issue_callback(&caller))
	}

	#[callback]
	fn sft_issue_callback(
		&self,
		caller: &Address,
		#[call_result] result: AsyncCallResult<TokenIdentifier>,
	) {
		match result {
			AsyncCallResult::Ok(token_identifier) => {
				self.storage_module()
					.last_issued_token()
					.set(&token_identifier);
				self.storage_module().last_error_message().clear();
			},
			AsyncCallResult::Err(message) => {
				// return issue cost to the caller
				let (returned_tokens, token_identifier) = self.call_value().payment_token_pair();
				if token_identifier.is_egld() && returned_tokens > 0 {
					self.send().direct_egld(caller, &returned_tokens, &[]);
				}

				self.storage_module()
					.last_error_message()
					.set(&message.err_msg);
			},
		}
	}
}
