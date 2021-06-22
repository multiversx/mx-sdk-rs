elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{
	function_selector::CurveArguments, function_selector::FunctionSelector,
	function_selector::Token,
};

#[elrond_wasm_derive::module]
pub trait StorageModule {
	#[view(getToken)]
	#[storage_mapper("token")]
	fn token(&self) -> MapMapper<Self::Storage, u64, TokenIdentifier>;

	#[view(lastErrorMessage)]
	#[storage_mapper("last_error_message")]
	fn last_error_message(&self) -> SingleValueMapper<Self::Storage, BoxedBytes>;

	#[view(getCurveFunction)]
	#[storage_mapper("bonding_curve")]
	fn bonding_curve(
		&self,
	) -> MapMapper<
		Self::Storage,
		Token,
		(
			FunctionSelector<Self::BigUint>,
			CurveArguments<Self::BigUint>,
		),
	>;
}
