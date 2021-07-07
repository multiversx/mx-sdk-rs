elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::common_methods::CallbackProxy;
use crate::utils::{
	events, storage,
	structs::{SupplyType, Token},
};

use super::common_methods;

const TOKEN_NUM_DECIMALS: usize = 18;
#[elrond_wasm_derive::module]
pub trait FungibleTokenModule:
	storage::StorageModule + events::EventsModule + common_methods::CommonMethods
{
	#[payable("EGLD")]
	#[allow(clippy::too_many_arguments)]
	#[endpoint(ftIssue)]
	fn issue(
		&self,
		#[payment] issue_cost: Self::BigUint,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
		initial_supply: Self::BigUint,
		supply_type: SupplyType<Self::BigUint>,
		accepted_payment: TokenIdentifier,
	) -> SCResult<AsyncCall<Self::SendApi>> {
		only_owner!(self, "only owner may call this function");

		let caller = self.blockchain().get_caller();

		Ok(ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.issue_fungible(
				issue_cost,
				&token_display_name,
				&token_ticker,
				&initial_supply,
				FungibleTokenProperties {
					num_decimals: TOKEN_NUM_DECIMALS,
					can_freeze: false,
					can_wipe: false,
					can_pause: false,
					can_mint: true,
					can_burn: false,
					can_change_owner: true,
					can_upgrade: true,
					can_add_special_roles: false,
				},
			)
			.async_call()
			.with_callback(self.callbacks().ft_issue_callback(
				caller,
				initial_supply,
				supply_type,
				accepted_payment,
			)))
	}

	#[endpoint(ftMint)]
	fn mint(
		&self,
		token_identifier: TokenIdentifier,
		amount: Self::BigUint,
	) -> SCResult<AsyncCall<Self::SendApi>> {
		only_owner!(self, "only owner may call this function");

		require!(
			!self
				.bonding_curve(&Token {
					identifier: token_identifier.clone(),
					nonce: 0u64,
				})
				.is_empty(),
			"Token not issued"
		);
		self.check_supply(
			&Token {
				identifier: token_identifier.clone(),
				nonce: 0u64,
			},
			&amount,
		)?;
		Ok(ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.mint(&token_identifier, &amount)
			.async_call()
			.with_callback(self.callbacks().mint_callback(token_identifier, &amount)))
	}

	#[endpoint(ftLocalMint)]
	fn local_mint(&self, token_identifier: TokenIdentifier, amount: Self::BigUint) -> SCResult<()> {
		self.check_supply(
			&Token {
				identifier: token_identifier.clone(),
				nonce: 0u64,
			},
			&amount,
		)?;
		self.send().esdt_local_mint(&token_identifier, &amount);
		self.bonding_curve(&Token {
			identifier: token_identifier,
			nonce: 0u64,
		})
		.update(|bonding_curve| {
			bonding_curve.arguments.available_supply += &amount;
			bonding_curve.arguments.balance += &amount;
		});
		Ok(())
	}

	#[endpoint(ftLocalBurn)]
	fn local_burn(&self, token_identifier: TokenIdentifier, amount: Self::BigUint) {
		self.send().esdt_local_burn(&token_identifier, &amount);
		self.bonding_curve(&Token {
			identifier: token_identifier,
			nonce: 0u64,
		})
		.update(|bonding_curve| {
			bonding_curve.arguments.available_supply -= &amount;
			bonding_curve.arguments.balance -= &amount;
		});
	}
}
