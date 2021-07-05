elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::utils::structs::{BondingCurve, Token};

#[elrond_wasm_derive::module]
pub trait StorageModule {
	#[storage_mapper("token_type")]
	fn token_type(
		&self,
		token: &TokenIdentifier,
	) -> SingleValueMapper<Self::Storage, EsdtTokenType>;

	#[storage_mapper("bonding_curve")]
	fn bonding_curve(
		&self,
		token: &Token,
	) -> SingleValueMapper<Self::Storage, BondingCurve<Self::BigUint>>;
}
