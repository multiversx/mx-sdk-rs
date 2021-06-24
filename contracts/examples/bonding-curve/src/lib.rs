#![no_std]
#![allow(unused_attributes)]
#![feature(trait_alias)]
#![feature(destructuring_assignment)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[path = "curves/curve_function.rs"]
mod curve_function;
use curve_function::CurveFunction;

mod function_selector;
use function_selector::{FunctionSelector, Token};
#[path = "utils/events.rs"]
mod events;
#[path = "curves/linear_function.rs"]
mod linear_function;
#[path = "utils/storage.rs"]
mod storage;

#[path = "tokens/common_methods.rs"]
mod common_methods;
#[path = "tokens/fungible_token.rs"]
mod fungible_token;
#[path = "tokens/non_fungible_token.rs"]
mod non_fungible_token;
#[path = "tokens/semi_fungible_token.rs"]
mod semi_fungible_token;

// This contract enables using a bonding curve for defining the behaviour of the price of the token as its balance changes.
//
// The contract allows issuing of any ESDT token and together with its issue elements such as details about the supply
// will be stored together with the Balance in an entity called CurveArguments. Because of working with different types of ESDT,
// the entity under which we will make the mapping with the curve function will be called Token, containing the TokenIdentifier and the nonce.
// The behaviour however is differend depending
// on the issued token:
// - FT:
//		* defines one bonding curve
//		* the nonce from Token should be set 0
//		* the supply and balance are indicated by the amount minted
// - SFT:
//		* defines multiple bonding curves (one per each nonce)
//		* the supply and balance are indicated by the amount of each nonce
// - NFT:
//		* defines one bonding curve
//		* the nonce from Token should be set 0
//		* the supply and balance are indicated by the number of nonces
//
// The bonding curve functions are set up in function_selector.rs
//
// When using this contract one should do the following process for each issued token:
//	- issue the token
//  - mint the token
//	- set the curve function

#[elrond_wasm_derive::contract]
pub trait BondingCurve:
	fungible_token::FTModule
	+ non_fungible_token::NFTModule
	+ semi_fungible_token::SFTModule
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
			.with_callback(BondingCurve::callbacks(self).change_roles_callback())
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
			.with_callback(BondingCurve::callbacks(self).change_roles_callback())
	}

	#[callback]
	fn change_roles_callback(&self, #[call_result] result: AsyncCallResult<()>) {
		match result {
			AsyncCallResult::Ok(()) => {
				self.last_error_message().clear();
			},
			AsyncCallResult::Err(message) => {
				self.last_error_message().set(&message.err_msg);
			},
		}
	}

	// when setting the bonding curve by a predefined function one mush pay attention by the parameters requested by the certain function.
	// All the predefined functions are set in the curves folder and are implementing the CurveFunction trait

	fn set_bonding_curve(&self, token: Token, function: FunctionSelector<Self::BigUint>) {
		self.bonding_curve(&token)
			.update(|(func, _, _)| *func = function);
	}

	#[view]
	fn check_sell_requirements(&self, token: &Token, amount: &Self::BigUint) -> SCResult<()> {
		require!(
			!self.bonding_curve(token).is_empty(),
			"Token is not issued yet!"
		);

		let (func, args, _) = self.bonding_curve(token).get();

		require!(
			func != FunctionSelector::None,
			"The token price was not set yet!"
		);

		require!(&args.balance >= amount, "Token provided not accepted");

		require!(
			amount > &Self::BigUint::zero(),
			"Must pay more than 0 tokens!"
		);
		Ok(())
	}

	#[view]
	fn check_buy_requirements(&self, token: &Token, amount: &Self::BigUint) -> SCResult<()> {
		let (func, _, payment) = self.bonding_curve(token).get();

		require!(
			func != FunctionSelector::None,
			"The token price was not set yet!"
		);

		if payment != token.identifier {
			return Err(SCError::from(BoxedBytes::from_concat(&[
				b"Only ",
				payment.as_esdt_identifier(),
				b" tokens accepted",
			])));
		}
		require!(
			amount > &Self::BigUint::zero(),
			"Must pay more than 0 tokens!"
		);
		Ok(())
	}

	#[payable("*")]
	#[endpoint(buyToken)]
	fn buy_token(
		&self,
		#[payment_amount] buy_amount: Self::BigUint,
		#[payment_token] identifier: TokenIdentifier,
		#[payment_nonce] nonce: u64,
	) -> SCResult<()> {
		let token = Token { identifier, nonce };
		self.check_buy_requirements(&token, &buy_amount)?;

		let calculated_price = self.bonding_curve(&token).update(|(func, args, _)| {
			let price = func.buy(buy_amount.clone(), args.clone());
			args.balance += buy_amount;
			price
		})?;
		let caller = self.blockchain().get_caller();

		self.send()
			.direct(&caller, &token.identifier, &calculated_price, b"buying");

		self.buy_token_event(&caller, &calculated_price);

		Ok(())
	}

	#[payable("*")]
	#[endpoint(sellToken)]
	fn sell_token(
		&self,
		#[payment_amount] sell_amount: Self::BigUint,
		#[payment_token] identifier: TokenIdentifier,
		#[payment_nonce] nonce: u64,
	) -> SCResult<()> {
		let token = Token { identifier, nonce };
		self.check_sell_requirements(&token, &sell_amount)?;
		let calculated_price = self.bonding_curve(&token).update(|(func, args, _)| {
			let price = func.buy(sell_amount.clone(), args.clone());
			args.balance -= sell_amount;
			price
		})?;

		let caller = self.blockchain().get_caller();

		self.send()
			.direct(&caller, &token.identifier, &calculated_price, b"selling");

		self.sell_token_event(&caller, &calculated_price);
		Ok(())
	}
}
