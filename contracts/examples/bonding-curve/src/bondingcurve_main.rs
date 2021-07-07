#![no_std]
#![allow(unused_attributes)]
#![feature(trait_alias)]
#![feature(destructuring_assignment)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod curves;
mod function_selector;
mod tokens;
mod utils;
use curves::curve_function::CurveFunction;
use function_selector::FunctionSelector;
use tokens::{common_methods, fungible_token, non_fungible_token, semi_fungible_token};
use utils::{
	events, storage,
	structs::{CurveArguments, Token},
};

#[elrond_wasm_derive::contract]
pub trait BondingCurveContract:
	fungible_token::FungibleTokenModule
	+ non_fungible_token::NonFungibleTokenModule
	+ semi_fungible_token::SemiFungibleTokenModule
	+ storage::StorageModule
	+ events::EventsModule
	+ common_methods::CommonMethods
{
	#[init]
	fn init(&self) {}

	#[endpoint(setLocalRoles)]
	fn set_local_roles(
		&self,
		address: Address,
		token_identifier: TokenIdentifier,
		#[var_args] roles: VarArgs<EsdtLocalRole>,
	) -> AsyncCall<Self::SendApi> {
		ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.set_special_roles(&address, &token_identifier, roles.as_slice())
			.async_call()
			.with_callback(BondingCurveContract::callbacks(self).change_roles_callback())
	}

	#[endpoint(unsetLocalRoles)]
	fn unset_local_roles(
		&self,
		address: Address,
		token_identifier: TokenIdentifier,
		#[var_args] roles: VarArgs<EsdtLocalRole>,
	) -> AsyncCall<Self::SendApi> {
		ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.unset_special_roles(&address, &token_identifier, roles.as_slice())
			.async_call()
			.with_callback(BondingCurveContract::callbacks(self).change_roles_callback())
	}

	#[callback]
	fn change_roles_callback(&self, #[call_result] result: AsyncCallResult<()>) -> SCResult<()> {
		match result {
			AsyncCallResult::Ok(()) => Ok(()),
			AsyncCallResult::Err(message) => Err(message.err_msg.into()),
		}
	}

	#[endpoint(setBondingCurve)]
	fn set_bonding_curve(
		&self,
		token: Token,
		function: FunctionSelector<Self::BigUint>,
	) -> SCResult<()> {
		require!(
			!self.bonding_curve(&token).is_empty(),
			"Token is not issued yet!"
		);
		self.bonding_curve(&token)
			.update(|bonding_curve| bonding_curve.curve = function);
		Ok(())
	}

	#[view]
	fn check_sell_requirements(
		&self,
		issued_token: &Token,
		amount: &Self::BigUint,
	) -> SCResult<()> {
		require!(
			!self.bonding_curve(issued_token).is_empty(),
			"Token is not issued yet!"
		);

		let bonding_curve = self.bonding_curve(issued_token).get();

		require!(
			bonding_curve.curve != FunctionSelector::None,
			"The token price was not set yet!"
		);

		require!(
			amount > &Self::BigUint::zero(),
			"Must pay more than 0 tokens!"
		);
		Ok(())
	}

	#[view]
	fn check_buy_requirements(
		&self,
		owned_token: &Token,
		exchanging_token: &Token,
		amount: &Self::BigUint,
	) -> SCResult<()> {
		let bonding_curve = self.bonding_curve(owned_token).get();

		require!(
			bonding_curve.curve != FunctionSelector::None,
			"The token price was not set yet!"
		);

		require!(
			amount > &Self::BigUint::zero(),
			"Must buy more than 0 tokens!"
		);

		self.check_given_token(
			&bonding_curve.accepted_payment,
			&exchanging_token.identifier,
		)
	}

	fn check_given_token(
		&self,
		accepted_token: &TokenIdentifier,
		given_token: &TokenIdentifier,
	) -> SCResult<()> {
		if given_token != accepted_token {
			return Err(SCError::from(BoxedBytes::from_concat(&[
				b"Only ",
				accepted_token.as_esdt_identifier(),
				b" tokens accepted",
			])));
		}
		Ok(())
	}

	#[payable("*")]
	#[endpoint(sellToken)]
	fn sell_token(
		&self,
		#[payment_amount] sell_amount: Self::BigUint,
		#[payment_token] offered_token: TokenIdentifier,
		#[payment_nonce] nonce: u64,
	) -> SCResult<()> {
		let owned_token = Token {
			identifier: offered_token,
			nonce,
		};
		self.check_sell_requirements(&owned_token, &sell_amount)?;

		let calculated_price = self.bonding_curve(&owned_token).update(|bonding_curve| {
			let price = self.sell(
				&bonding_curve.curve,
				sell_amount.clone(),
				bonding_curve.arguments.clone(),
			);
			bonding_curve.arguments.balance += sell_amount;
			price
		})?;

		let caller = self.blockchain().get_caller();

		self.send().direct(
			&caller,
			&self.bonding_curve(&owned_token).get().accepted_payment,
			&calculated_price,
			b"selling",
		);

		self.sell_token_event(&caller, &calculated_price);

		Ok(())
	}

	#[payable("*")]
	#[endpoint(buyToken)]
	fn buy_token(
		&self,
		#[payment_amount] payment: Self::BigUint,
		#[payment_token] offered_token: TokenIdentifier,
		requested_amount: Self::BigUint,
		requested_token: TokenIdentifier,
		#[var_args] requested_nonce: OptionalArg<u64>,
	) -> SCResult<()> {
		let exchanging_token = Token {
			identifier: offered_token,
			nonce: 0u64,
		};
		let token_type = self.token_type(&requested_token).get();
		let mut desired_nonce = 0u64;
		let owned_token;

		if token_type != EsdtTokenType::Fungible {
			desired_nonce = requested_nonce
				.into_option()
				.ok_or("Expected nonce for the desired NFT")?;
		}
		if token_type == EsdtTokenType::SemiFungible {
			owned_token = Token {
				identifier: requested_token,
				nonce: desired_nonce,
			};
		} else {
			owned_token = Token {
				identifier: requested_token,
				nonce: 0u64,
			};
		}
		self.check_buy_requirements(&owned_token, &exchanging_token, &requested_amount)?;

		let calculated_price = self.bonding_curve(&owned_token).update(|bonding_curve| {
			let price = self.buy(
				&bonding_curve.curve,
				requested_amount.clone(),
				bonding_curve.arguments.clone(),
			);
			require!(
				price.clone()? <= payment,
				"The payment provided is not enough for the transaction"
			);

			bonding_curve.arguments.balance -= &requested_amount;

			price
		})?;

		let caller = self.blockchain().get_caller();

		if token_type == EsdtTokenType::Fungible {
			self.send().direct(
				&caller,
				&owned_token.identifier,
				&requested_amount,
				b"buying",
			);
		} else {
			self.send().direct_nft(
				&caller,
				&(owned_token.identifier),
				desired_nonce,
				&requested_amount,
				b"buying",
			);
		}

		self.send().direct(
			&caller,
			&exchanging_token.identifier,
			&(&payment - &calculated_price),
			b"rest",
		);

		self.buy_token_event(&caller, &calculated_price);
		Ok(())
	}

	fn buy(
		&self,
		function_selector: &FunctionSelector<Self::BigUint>,
		amount: Self::BigUint,
		arguments: CurveArguments<Self::BigUint>,
	) -> SCResult<Self::BigUint> {
		let token_start = arguments.first_token_available();
		function_selector.calculate_price(&token_start, &amount, &arguments)
	}

	fn sell(
		&self,
		function_selector: &FunctionSelector<Self::BigUint>,
		amount: Self::BigUint,
		arguments: CurveArguments<Self::BigUint>,
	) -> SCResult<Self::BigUint> {
		let token_start = &arguments.first_token_available() - &amount;
		function_selector.calculate_price(&token_start, &amount, &arguments)
	}
}
