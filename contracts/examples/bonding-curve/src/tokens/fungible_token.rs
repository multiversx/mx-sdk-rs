elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{
	events,
	function_selector::FunctionSelector,
	function_selector::SupplyType,
	function_selector::{CurveArguments, Token},
	storage,
};

const TOKEN_NUM_DECIMALS: usize = 0;
#[elrond_wasm_derive::module]
pub trait FTModule: storage::StorageModule + events::EventsModule {
	#[payable("EGLD")]
	#[endpoint(ftIssue)]
	fn ft_issue(
		&self,
		#[payment] issue_cost: Self::BigUint,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
		initial_supply: Self::BigUint,
		supply_type: SupplyType,
		maximum_suply: Self::BigUint,
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
				maximum_suply,
			)))
	}
	#[callback]
	fn ft_issue_callback(
		&self,
		caller: Address,
		initial_supply: Self::BigUint,
		supply_type: SupplyType,
		maximum_supply: Self::BigUint,
		#[payment_token] token_identifier: TokenIdentifier,
		#[payment] amount: Self::BigUint,
		#[call_result] result: AsyncCallResult<()>,
	) {
		match result {
			AsyncCallResult::Ok(()) => {
				self.bonding_curve(&Token {
					identifier: token_identifier.clone(),
					nonce: 0u64,
				})
				.set(&(
					FunctionSelector::None,
					CurveArguments {
						supply_type,
						max_supply: maximum_supply,
						available_supply: initial_supply.clone(),
						balance: initial_supply,
					},
				));
				self.last_error_message().clear();
			},
			AsyncCallResult::Err(message) => {
				if token_identifier.is_egld() && amount > 0 {
					self.send().direct_egld(&caller, &amount, &[]);
				}

				self.last_error_message().set(&message.err_msg);
			},
		}
	}

	#[endpoint(ftmint)]
	fn ft_mint(
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

		let (_, args) = self
			.bonding_curve(&Token {
				identifier: token_identifier.clone(),
				nonce: 0u64,
			})
			.get();

		require!(
			args.supply_type == SupplyType::Unlimited || args.available_supply < args.max_supply,
			"Maximum supply limit reached!"
		);

		require!(
			args.supply_type == SupplyType::Unlimited
				|| args.available_supply + amount.clone() <= args.max_supply,
			"Minting will exceed the maximum supply limit!"
		);

		Ok(ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.mint(&token_identifier, &amount)
			.async_call()
			.with_callback(self.callbacks().ft_mint_callback(token_identifier, &amount)))
	}

	#[callback]
	fn ft_mint_callback(
		&self,
		token_identifier: TokenIdentifier,
		amount: &Self::BigUint,
		#[call_result] result: AsyncCallResult<()>,
	) {
		match result {
			AsyncCallResult::Ok(()) => {
				self.bonding_curve(&Token {
					identifier: token_identifier.clone(),
					nonce: 0u64,
				})
				.update(|(_, args)| {
					args.available_supply += amount;
					args.balance += amount;
				});
			},
			AsyncCallResult::Err(message) => {
				self.last_error_message().set(&message.err_msg);
			},
		}
	}

	#[endpoint(ftLocalMint)]
	fn ft_local_mint(&self, token_identifier: TokenIdentifier, amount: Self::BigUint) {
		self.send().esdt_local_mint(&token_identifier, &amount);
		self.bonding_curve(&Token {
			identifier: token_identifier.clone(),
			nonce: 0u64,
		})
		.update(|(_, args)| {
			args.available_supply += &amount;
			args.balance += &amount;
		});
	}

	#[endpoint(ftLocalBurn)]
	fn ft_local_burn(&self, token_identifier: TokenIdentifier, amount: Self::BigUint) {
		self.send().esdt_local_burn(&token_identifier, &amount);
		self.bonding_curve(&Token {
			identifier: token_identifier.clone(),
			nonce: 0u64,
		})
		.update(|(_, args)| {
			args.available_supply -= &amount;
			args.balance -= &amount;
		});
	}
}
