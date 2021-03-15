use elrond_wasm::{api::BigUintApi, types::TokenIdentifier};
use elrond_wasm::types::Address;

elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct DepositInfo<BigUint: BigUintApi>
{
	pub amount : BigUint,
	pub depositor_address : Address,
	pub expiration: u64,
	pub token_name : TokenIdentifier
}		
