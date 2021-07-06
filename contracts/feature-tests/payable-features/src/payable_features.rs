#![no_std]

elrond_wasm::imports!();

/// Contract that only tests the call value features,
/// i.e. the framework/Arwen functionality for accepting EGLD and ESDT payments.
#[elrond_wasm_derive::contract]
pub trait PayableFeatures {
	#[view]
	#[payable("*")]
	fn check_call_value(
		&self,
	) -> MultiResult5<Self::BigUint, Self::BigUint, TokenIdentifier, Self::BigUint, TokenIdentifier>
	{
		let (pair_call_value, pair_token_name) = self.call_value().payment_token_pair();
		(
			self.call_value().egld_value(),
			self.call_value().esdt_value(),
			self.call_value().token(),
			pair_call_value,
			pair_token_name,
		)
			.into()
	}

	#[endpoint]
	#[payable("*")]
	fn payable_any_1(
		&self,
		#[payment] payment: Self::BigUint,
		#[payment_token] token: TokenIdentifier,
	) -> MultiResult2<Self::BigUint, TokenIdentifier> {
		(payment, token).into()
	}

	#[endpoint]
	#[payable("*")]
	fn payable_any_2(
		&self,
		#[payment] payment: Self::BigUint,
	) -> MultiResult2<Self::BigUint, TokenIdentifier> {
		let token = self.call_value().token();
		(payment, token).into()
	}

	#[endpoint]
	#[payable("*")]
	fn payable_any_3(
		&self,
		#[payment_token] token: TokenIdentifier,
	) -> MultiResult2<Self::BigUint, TokenIdentifier> {
		let (payment, _) = self.call_value().payment_token_pair();
		(payment, token).into()
	}

	#[endpoint]
	#[payable("*")]
	fn payable_any_4(&self) -> MultiResult2<Self::BigUint, TokenIdentifier> {
		self.call_value().payment_token_pair().into()
	}

	#[endpoint]
	#[payable("EGLD")]
	fn payable_egld_1(
		&self,
		#[payment_token] token: TokenIdentifier,
	) -> MultiResult2<Self::BigUint, TokenIdentifier> {
		let payment = self.call_value().egld_value();
		(payment, token).into()
	}

	#[endpoint]
	#[payable("EGLD")]
	fn payable_egld_2(
		&self,
		#[payment] payment: Self::BigUint,
	) -> MultiResult2<Self::BigUint, TokenIdentifier> {
		let token = self.call_value().token();
		(payment, token).into()
	}

	#[endpoint]
	#[payable("EGLD")]
	fn payable_egld_3(
		&self,
		#[payment_token] token: TokenIdentifier,
	) -> MultiResult2<Self::BigUint, TokenIdentifier> {
		let payment = self.call_value().egld_value();
		(payment, token).into()
	}

	#[endpoint]
	#[payable("EGLD")]
	fn payable_egld_4(&self) -> MultiResult2<Self::BigUint, TokenIdentifier> {
		let payment = self.call_value().egld_value();
		let token = self.call_value().token();
		(payment, token).into()
	}

	#[endpoint]
	#[payable("PAYABLE-FEATURES-TOKEN")]
	fn payable_token_1(
		&self,
		#[payment] payment: Self::BigUint,
		#[payment_token] token: TokenIdentifier,
	) -> MultiResult2<Self::BigUint, TokenIdentifier> {
		(payment, token).into()
	}

	#[endpoint]
	#[payable("PAYABLE-FEATURES-TOKEN")]
	fn payable_token_2(
		&self,
		#[payment] payment: Self::BigUint,
	) -> MultiResult2<Self::BigUint, TokenIdentifier> {
		let token = self.call_value().token();
		(payment, token).into()
	}

	#[endpoint]
	#[payable("PAYABLE-FEATURES-TOKEN")]
	fn payable_token_3(
		&self,
		#[payment_token] token: TokenIdentifier,
	) -> MultiResult2<Self::BigUint, TokenIdentifier> {
		let payment = self.call_value().esdt_value();
		(payment, token).into()
	}

	#[endpoint]
	#[payable("PAYABLE-FEATURES-TOKEN")]
	fn payable_token_4(&self) -> MultiResult2<Self::BigUint, TokenIdentifier> {
		let payment = self.call_value().esdt_value();
		let token = self.call_value().token();
		(payment, token).into()
	}
}
