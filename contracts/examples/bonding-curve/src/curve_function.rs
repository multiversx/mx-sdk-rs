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
	fn first_token_available(&self) -> BigUint {
		&self.current_supply - &self.balance
	}
}

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone)]
pub struct Token {
	pub name: BoxedBytes,
	pub identifier: TokenIdentifier,
}

pub trait CurveFunction<BigUint: BigUintApi>
where
	for<'a, 'b> &'a BigUint: core::ops::Add<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: core::ops::Sub<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: core::ops::Mul<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: core::ops::Div<&'b BigUint, Output = BigUint>,
	for<'b> BigUint: core::ops::AddAssign<&'b BigUint>,
	for<'b> BigUint: core::ops::SubAssign<&'b BigUint>,
	for<'b> BigUint: core::ops::MulAssign<&'b BigUint>,
	for<'b> BigUint: core::ops::DivAssign<&'b BigUint>,
	BigUint: BigUintApi,
{
	fn function(
		&self,
		token_start: BigUint,
		amount: BigUint,
		arguments: &CurveArguments<BigUint>,
	) -> SCResult<BigUint>;

	fn sell(&self, amount: BigUint, arguments: CurveArguments<BigUint>) -> SCResult<BigUint> {
		let token_start = arguments.first_token_available();
		self.function(token_start, amount, &arguments)
	}

	fn buy(&self, amount: BigUint, arguments: CurveArguments<BigUint>) -> SCResult<BigUint> {
		let token_start = &arguments.first_token_available() - &amount - BigUint::from(1u64);
		self.function(token_start, amount, &arguments)
	}
}
