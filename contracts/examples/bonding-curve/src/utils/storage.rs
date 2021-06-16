elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{curve_arguments::SupplyType, curves_setup::CurvesSetup};

#[elrond_wasm_derive::module]
pub trait StorageModule {
	#[view(supply_type)]
	#[storage_mapper("supply_type")]
	fn supply_type(&self) -> SingleValueMapper<Self::Storage, SupplyType>;

	#[view(max_supply)]
	#[storage_mapper("max_supply")]
	fn max_supply(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

	#[view(getMintedSupply)]
	#[storage_mapper("supply")]
	fn supply(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

	#[view(getCurves)]
	#[storage_mapper("curves")]
	fn curves(&self) -> SingleValueMapper<Self::Storage, CurvesSetup<Self::BigUint>>;

	#[view(getAvailableSupply)]
	#[storage_mapper("balance")]
	fn balance(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

	#[view(getExchangingToken)]
	#[storage_mapper("exchanging_token")]
	fn exchanging_token(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;
}
