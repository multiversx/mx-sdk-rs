elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::curve_function::CurveFunction;
use crate::linear_function::LinearFunction;
use crate::power_function::PowerFunction;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum SupplyType {
	Limited,
	Unlimited,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub struct CurveArguments<BigUint: BigUintApi> {
	pub supply_type: SupplyType,
	pub max_supply: BigUint,
	pub available_supply: BigUint,
	pub balance: BigUint,
}

impl<BigUint> CurveArguments<BigUint>
where
	for<'a, 'b> &'a BigUint: core::ops::Sub<&'b BigUint, Output = BigUint>,
	for<'b> BigUint: core::ops::SubAssign<&'b BigUint>,
	BigUint: BigUintApi,
{
	pub fn first_token_available(&self) -> BigUint {
		&self.available_supply - &self.balance
	}
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub struct Token {
	pub identifier: TokenIdentifier,
	pub nonce: u64,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum FunctionSelector<BigUint>
where
	BigUint: BigUintApi,
{
	Linear(LinearFunction<BigUint>),
	Power(PowerFunction<BigUint>),
	Custom1(BigUint),
	None,
}

impl<BigUint> CurveFunction<BigUint> for FunctionSelector<BigUint>
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
	) -> SCResult<BigUint> {
		match &self {
			FunctionSelector::Linear(linear_function) => {
				CurveFunction::function(linear_function, token_start, amount, arguments)
			},
			FunctionSelector::Power(power_function) => {
				CurveFunction::function(power_function, token_start, amount, arguments)
			},

			FunctionSelector::Custom1(initial_cost) => {
				let sum = token_start + amount;
				Ok(
					&(&sum * &sum * sum / BigUint::from(3u64)) - &arguments.balance
						+ initial_cost.clone(),
				)
			},
			FunctionSelector::None => Err("Bonding Curve function is not assiged".into()),
		}
	}
}
