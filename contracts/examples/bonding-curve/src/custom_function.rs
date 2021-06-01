elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::bc_function::{BCFunction, CurveArguments};

pub struct CustomFunction<BigUint: BigUintApi> {}

impl<BigUint> BCFunction<BigUint> for CustomFunction<BigUint>
where
	BigUint: BigUintApi,
{
	fn function(
		&self,
		token_start: BigUint,
		amount: BigUint,
		arguments: &CurveArguments<BigUint>,
	) -> SCResult<BigUint> {
		Ok(
			(token_start + amount) * (token_start + amount) * (token_start + amount)
				/ BigUint::from(3u64)
				- arguments.balance,
		)
	}
}
