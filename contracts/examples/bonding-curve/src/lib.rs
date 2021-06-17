#![no_std]
#![allow(unused_attributes)]
#![feature(trait_alias)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod curve_arguments;
use curve_arguments::CurveArguments;

#[path = "curves/curve_function.rs"]
mod curve_function;
use curve_function::CurveFunction;
#[path = "curves/linear_function.rs"]
mod linear_function;
#[path = "curves/power_function.rs"]
mod power_function;

mod curves_setup;
#[path = "utils/events.rs"]
mod events;
#[path = "tokens/fungible_token.rs"]
mod fungible_token;
#[path = "tokens/non_fungible_token.rs"]
mod non_fungible_token;
#[path = "tokens/semi_fungible_token.rs"]
mod semi_fungible_token;
#[path = "utils/storage.rs"]
mod storage;

#[elrond_wasm_derive::contract]
pub trait BondingCurve:
	fungible_token::FTModule
	+ non_fungible_token::NFTModule
	+ semi_fungible_token::SFTModule
	+ storage::StorageModule
	+ events::EventsModule
{
	#[init]
	fn init(&self) {}

	#[view]
	fn check_sell_requirements(
		&self,
		token: &TokenIdentifier,
		amount: &Self::BigUint,
	) -> SCResult<()> {
		let issued_token = self.issued_token().get();
		if &issued_token != token {
			return Err(SCError::from(BoxedBytes::from_concat(&[
				b"Only ",
				token.as_esdt_identifier(),
				b" tokens accepted",
			])));
		}
		if &self.balance().get() < amount {
			return Err(SCError::from(BoxedBytes::from_concat(&[
				b"Contract does not have enough ",
				token.as_esdt_identifier(),
				b". Please try again once more is minted.",
			])));
		}
		require!(
			amount > &Self::BigUint::zero(),
			"Must pay more than 0 tokens!"
		);
		Ok(())
	}

	fn get_curve_arguments(self) -> CurveArguments<Self::BigUint> {
		CurveArguments {
			supply_type: self.supply_type().get(),
			max_supply: self.max_supply().get(),
			current_supply: self.supply().get(),
			balance: self.balance().get(),
		}
	}

	#[view]
	fn check_buy_requirements(
		&self,
		token: &TokenIdentifier,
		amount: &Self::BigUint,
	) -> SCResult<()> {
		let exchanging_token = &self.exchanging_token().get();
		if exchanging_token != token {
			return Err(SCError::from(BoxedBytes::from_concat(&[
				b"Only ",
				exchanging_token.as_esdt_identifier(),
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
		#[payment] buy_amount: Self::BigUint,
		#[payment_token] token_identifier: TokenIdentifier,
	) -> SCResult<()> {
		self.check_buy_requirements(&token_identifier, &buy_amount)?;

		let calculated_price = self
			.curves()
			.get()
			.buy(buy_amount.clone(), self.get_curve_arguments())
			.unwrap();

		self.balance().update(|balance| *balance += buy_amount);

		let caller = self.blockchain().get_caller();

		self.send()
			.direct(&caller, &token_identifier, &calculated_price, b"buying");

		self.buy_token_event(&caller, &calculated_price);

		Ok(())
	}

	#[payable("*")]
	#[endpoint(sellToken)]
	fn sell_token(
		&self,
		#[payment] sell_amount: Self::BigUint,
		#[payment_token] token_identifier: TokenIdentifier,
	) -> SCResult<()> {
		self.check_sell_requirements(&token_identifier, &sell_amount)?;

		let calculated_price = self
			.curves()
			.get()
			.sell(sell_amount.clone(), self.get_curve_arguments())
			.unwrap();

		self.balance().update(|balance| *balance -= sell_amount);

		let caller = self.blockchain().get_caller();

		self.send()
			.direct(&caller, &token_identifier, &calculated_price, b"selling");

		self.sell_token_event(&caller, &calculated_price);
		Ok(())
	}
}
