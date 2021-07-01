elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{
	events,
	function_selector::FunctionSelector,
	storage,
	utils::structs::{BondingCurve, CurveArguments, SupplyType, Token},
};

const TOKEN_NUM_DECIMALS: usize = 18;
#[elrond_wasm_derive::module]
pub trait FungibleTokenModule: storage::StorageModule + events::EventsModule {
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
			.with_callback(self.callbacks().issue_callback(
				caller,
				initial_supply,
				supply_type,
				accepted_payment,
			)))
	}

	#[callback]
	#[allow(clippy::too_many_arguments)]
	fn issue_callback(
		&self,
		caller: Address,
		initial_supply: Self::BigUint,
		supply_type: SupplyType<Self::BigUint>,
		accepted_payment: TokenIdentifier,
		#[payment_token] token_identifier: TokenIdentifier,
		#[payment] amount: Self::BigUint,
		#[call_result] result: AsyncCallResult<()>,
	) -> SCResult<()> {
		match result {
			AsyncCallResult::Ok(()) => {
				self.bonding_curve(&Token {
					identifier: token_identifier,
					nonce: 0u64,
				})
				.set(&BondingCurve {
					curve: FunctionSelector::None,
					arguments: CurveArguments {
						supply_type,
						available_supply: initial_supply.clone(),
						balance: initial_supply,
					},
					accepted_payment,
				});
				Ok(())
			},
			AsyncCallResult::Err(message) => {
				if token_identifier.is_egld() && amount > 0 {
					self.send().direct_egld(&caller, &amount, &[]);
				}

				Err(message.err_msg.into())
			},
		}
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

		let bonding_curve = self
			.bonding_curve(&Token {
				identifier: token_identifier.clone(),
				nonce: 0u64,
			})
			.get();

		if bonding_curve.arguments.supply_type != SupplyType::Unlimited {
			require!(
				bonding_curve.arguments.available_supply
					< bonding_curve.arguments.supply_type.get_limit()?,
				"Maximum supply limit reached!"
			);

			require!(
				bonding_curve.arguments.available_supply + amount.clone()
					<= bonding_curve.arguments.supply_type.get_limit()?,
				"Minting will exceed the maximum supply limit!"
			);
		}
		Ok(ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.mint(&token_identifier, &amount)
			.async_call()
			.with_callback(self.callbacks().mint_callback(token_identifier, &amount)))
	}

	#[callback]
	fn mint_callback(
		&self,
		token_identifier: TokenIdentifier,
		amount: &Self::BigUint,
		#[call_result] result: AsyncCallResult<()>,
	) -> SCResult<()> {
		match result {
			AsyncCallResult::Ok(()) => {
				self.bonding_curve(&Token {
					identifier: token_identifier,
					nonce: 0u64,
				})
				.update(|bonding_curve| {
					bonding_curve.arguments.available_supply += amount;
					bonding_curve.arguments.balance += amount;
				});
				Ok(())
			},
			AsyncCallResult::Err(message) => Err(message.err_msg.into()),
		}
	}

	#[endpoint(ftLocalMint)]
	fn local_mint(&self, token_identifier: TokenIdentifier, amount: Self::BigUint) {
		self.send().esdt_local_mint(&token_identifier, &amount);
		self.bonding_curve(&Token {
			identifier: token_identifier,
			nonce: 0u64,
		})
		.update(|bonding_curve| {
			bonding_curve.arguments.available_supply += &amount;
			bonding_curve.arguments.balance += &amount;
		});
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
