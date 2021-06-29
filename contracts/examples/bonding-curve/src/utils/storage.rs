elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::utils::structs::{BondingCurve, Token};

#[elrond_wasm_derive::module]
pub trait StorageModule {
	#[view(lastErrorMessage)]
	#[storage_mapper("last_error_message")]
	fn last_error_message(&self) -> SingleValueMapper<Self::Storage, BoxedBytes>;

	#[storage_mapper("bonding_curve")]
	fn bonding_curve(
		&self,
		token: &Token,
	) -> SingleValueMapper<Self::Storage, BondingCurve<Self::BigUint>>;
}
