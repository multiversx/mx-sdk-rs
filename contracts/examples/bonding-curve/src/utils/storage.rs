elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::utils::structs::{BondingCurve, Token};

use super::structs::TokenOwnershipDetails;

#[elrond_wasm_derive::module]
pub trait StorageModule {
	fn get_token_nonce_ranges(&self, identifier: &TokenIdentifier) -> (EsdtTokenType, Vec<u64>) {
		let details = self.token_details(identifier).get();

		(details.token_type, details.token_nonces)
	}

	#[storage_mapper("token_details")]
	fn token_details(
		&self,
		token: &TokenIdentifier,
	) -> SingleValueMapper<Self::Storage, TokenOwnershipDetails>;

	#[storage_mapper("bonding_curve")]
	fn bonding_curve(
		&self,
		token: &Token,
	) -> SingleValueMapper<Self::Storage, BondingCurve<Self::BigUint>>;

	#[storage_mapper("owned_tokens")]
	fn owned_tokens(
		&self,
		owner: &Address,
	) -> SingleValueMapper<Self::Storage, Vec<TokenIdentifier>>;
}
