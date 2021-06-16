elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum SupplyType {
	Limited,
	Unlimited,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub struct CurveArguments<BigUint: BigUintApi> {
	pub supply_type: SupplyType,
	pub max_supply: BigUint,
	pub current_supply: BigUint,
	pub balance: BigUint,
}

impl<BigUint> CurveArguments<BigUint>
where
	for<'a, 'b> &'a BigUint: core::ops::Sub<&'b BigUint, Output = BigUint>,
	for<'b> BigUint: core::ops::SubAssign<&'b BigUint>,
	BigUint: BigUintApi,
{
	pub fn first_token_available(&self) -> BigUint {
		&self.current_supply - &self.balance
	}
}
