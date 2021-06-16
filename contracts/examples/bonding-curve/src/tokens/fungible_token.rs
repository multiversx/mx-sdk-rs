elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{curve_arguments::SupplyType, events, storage};

const TOKEN_NUM_DECIMALS: usize = 18;
#[elrond_wasm_derive::module]
pub trait FungibleTokenModule: storage::StorageModule + events::EventsModule {
	#[payable("*")]
	#[endpoint(issueFungibleToken)]
	fn issue_fungible_token(
		&self,
		#[payment] issue_cost: Self::BigUint,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
		initial_supply: Self::BigUint,
	) -> SCResult<AsyncCall<Self::SendApi>> {
		only_owner!(self, "only owner may call this function");

		let caller = self.blockchain().get_caller();

		self.issue_started_event(&caller, token_ticker.as_slice(), &initial_supply);

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
			.with_callback(self.callbacks().ft_issue_callback(caller)))
	}

	#[callback]
	fn ft_issue_callback(
		&self,
		caller: Address,
		#[payment_token] token_identifier: TokenIdentifier,
		#[payment] amount: Self::BigUint,
		#[call_result] result: AsyncCallResult<()>,
	) {
		match result {
			AsyncCallResult::Ok(()) => {
				self.issued_token().set(&token_identifier);
				self.issue_success_event(&caller, &token_identifier, &amount);
				self.supply().set(&amount);
				self.balance().set(&amount);
			},
			AsyncCallResult::Err(message) => {
				self.issue_failure_event(&caller, message.err_msg.as_slice());
				if token_identifier.is_egld() && amount > 0 {
					self.send().direct_egld(&caller, &amount, &[]);
				}
			},
		}
	}

	#[endpoint(mintToken)]
	fn mint_token(
		&self,
		token_identifier: TokenIdentifier,
		amount: Self::BigUint,
	) -> SCResult<AsyncCall<Self::SendApi>> {
		only_owner!(self, "only owner may call this function");

		let supply_type = self.supply_type().get();
		let supply = self.supply().get();
		let max_supply = self.max_supply().get();
		require!(
			!self.issued_token().is_empty(),
			"Must issue token before minting"
		);

		require!(
			supply_type == SupplyType::Unlimited || supply < max_supply,
			"Maximum supply limit reached!"
		);

		require!(
			supply_type == SupplyType::Unlimited || supply + amount.clone() <= max_supply,
			"Minting will exceed the maximum supply limit!"
		);

		let caller = self.blockchain().get_caller();
		self.mint_started_event(&caller, &amount);

		Ok(ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.mint(&token_identifier, &amount)
			.async_call()
			.with_callback(self.callbacks().mint_callback(caller, &amount)))
	}

	#[callback]
	fn mint_callback(
		&self,
		caller: Address,
		amount: &Self::BigUint,
		#[call_result] result: AsyncCallResult<()>,
	) {
		match result {
			AsyncCallResult::Ok(()) => {
				self.mint_success_event(&caller);
				self.supply().update(|supply| *supply += amount);
				self.balance().update(|balance| *balance += amount);
			},
			AsyncCallResult::Err(message) => {
				self.mint_failure_event(&caller, message.err_msg.as_slice());
			},
		}
	}
}
