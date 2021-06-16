elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::curve_arguments::CurveArguments;
use crate::curve_function::CurveFunction;
use crate::linear_function::LinearFunction;
use crate::power_function::PowerFunction;

#[derive(TopDecode, TopEncode, TypeAbi)]
pub enum CurvesSetup<BigUint>
where
	BigUint: BigUintApi,
{
	Linear(LinearFunction<BigUint>),
	Power(PowerFunction<BigUint>),
	Custom1(()),
}

impl<BigUint> CurveFunction<BigUint> for CurvesSetup<BigUint>
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
			CurvesSetup::Linear(linear_function) => {
				CurveFunction::function(linear_function, token_start, amount, arguments)
			},
			CurvesSetup::Power(power_function) => {
				CurveFunction::function(power_function, token_start, amount, arguments)
			},

			CurvesSetup::Custom1(()) => {
				let sum = token_start + amount;
				Ok(&(&sum * &sum * sum / BigUint::from(3u64)) - &arguments.balance)
			},
		}
	}
}
