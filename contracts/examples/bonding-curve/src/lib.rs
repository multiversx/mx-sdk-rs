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
#[path = "curves/linear_function.rs"]
mod linear_function;
#[path = "curves/power_function.rs"]
mod power_function;

#[path = "utils/events.rs"]
mod events;
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

	fn set_bonding_curve(&self, token: Token, function: FunctionSelector<Self::BigUint>) {
		self.bonding_curve(&token)
			.update(|(func, _)| *func = function);
	}

	#[view]
	fn check_sell_requirements(&self, token: &Token, amount: &Self::BigUint) -> SCResult<()> {
		require!(
			self.bonding_curve(token).is_empty(),
			"Token provided not accepted"
		);

		let (_, args) = self.bonding_curve(token).get();

		require!(&args.balance >= amount, "Token provided not accepted");

		require!(
			amount > &Self::BigUint::zero(),
			"Must pay more than 0 tokens!"
		);
		Ok(())
	}

	#[view]
	fn check_buy_requirements(&self, token: &Token, amount: &Self::BigUint) -> SCResult<()> {
		if !self.bonding_curve(token).is_empty() {
			return Err(SCError::from(BoxedBytes::from_concat(&[
				b"Only ",
				token.identifier.as_esdt_identifier(),
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

		let calculated_price = self.bonding_curve(&token).update(|(func, args)| {
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
		let calculated_price = self.bonding_curve(&token).update(|(func, args)| {
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
