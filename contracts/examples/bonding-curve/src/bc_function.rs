elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum SupplyType {
	Limited,
	Limitless,
}

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone)]
pub struct CurveArguments<BigUint: BigUintApi> {
	pub supply_type: SupplyType,
	pub ratio: BigUint,
	pub max_supply: BigUint,
	pub max_ratio: BigUint,
	pub balance: BigUint,
}

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone)]
pub struct Token {
	pub name: BoxedBytes,
	pub identifier: TokenIdentifier,
}
