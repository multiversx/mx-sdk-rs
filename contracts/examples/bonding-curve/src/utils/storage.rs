elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::structs::BondingCurve;
use super::structs::TokenOwnershipData;

#[elrond_wasm_derive::module]
pub trait StorageModule {
	#[storage_mapper("token_details")]
	fn token_details(
		&self,
		token: &TokenIdentifier,
	) -> SingleValueMapper<Self::Storage, TokenOwnershipData>;

	#[storage_mapper("bonding_curve")]
	fn bonding_curve(
		&self,
		token: &TokenIdentifier,
	) -> SingleValueMapper<Self::Storage, BondingCurve<Self::BigUint>>;

	#[storage_mapper("owned_tokens")]
	fn owned_tokens(&self, owner: &Address) -> SetMapper<Self::Storage, TokenIdentifier>;

	#[storage_mapper("nonce_amount")]
	fn nonce_amount(
		&self,
		identifier: &TokenIdentifier,
		nonce: u64,
	) -> SingleValueMapper<Self::Storage, Self::BigUint>;

	#[storage_mapper("sell_availability")]
	fn sell_availability(&self) -> SingleValueMapper<Self::Storage, bool>;
}
