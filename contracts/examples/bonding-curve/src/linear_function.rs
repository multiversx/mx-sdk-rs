elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::bc_function::{BCFunction, CurveArguments};

// logic is ax + b  = y
// x - issued token
// y - exchanging token
// a & b - linear equation coeficients
// because of selling n tokens at a time, the total price y' will be
// a* (nx + (n - 1) n / 2) + nb = y'

pub struct LinearFunction<BigUint: BigUintApi> {
	pub a: BigUint,
	pub b: BigUint,
}

impl<BigUint> BCFunction<BigUint> for LinearFunction<BigUint>
where
	BigUint: BigUintApi,
{
	fn function(
		&self,
		token_start: BigUint,
		amount: BigUint,
		_arguments: &CurveArguments<BigUint>,
	) -> SCResult<BigUint> {
		Ok(self.a.clone() * sum_interval(token_start, amount.clone()) + self.b.clone() * amount)
	}
}
fn sum_interval<BigUint: BigUintApi>(n: BigUint, x: BigUint) -> BigUint {
	x * n.clone() + (n.clone() - BigUint::from(1u64)) * n / BigUint::from(2u64)
}
