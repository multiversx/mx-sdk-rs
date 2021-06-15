elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::bc_function::{CurveArguments, CurveFunction};

// logic is ax + b  = y
// x - issued token
// y - exchanging token
// a - linear_coefficient
// b - constant_coefficient
// because of selling n tokens at a time, the total price y' will be
// a* (nx + (n - 1) n / 2) + nb = y'

pub struct LinearFunction<BigUint: BigUintApi> {
	pub linear_coefficient: BigUint,
	pub constant_coefficient: BigUint,
}

impl<BigUint> CurveFunction<BigUint> for LinearFunction<BigUint>
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
		_arguments: &CurveArguments<BigUint>,
	) -> SCResult<BigUint> {
		Ok(
			&self.linear_coefficient * &sum_interval(&token_start, &amount)
				+ &self.constant_coefficient * &amount,
		)
	}
}
fn sum_interval<BigUint: BigUintApi>(n: &BigUint, x: &BigUint) -> BigUint
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
	x * n + &(n - &BigUint::from(1u64)) * n / BigUint::from(2u64)
}
