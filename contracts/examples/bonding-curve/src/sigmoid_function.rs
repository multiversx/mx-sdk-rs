elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::bc_function::{BCFunction, CurveArguments};

pub struct SigmoidFunction<BigUint: BigUintApi> {}

impl<BigUint> BCFunction<BigUint> for SigmoidFunction<BigUint>
where
	BigUint: BigUintApi,
{
	fn function(
		&self,
		token_start: BigUint,
		amount: BigUint,
		_arguments: &CurveArguments<BigUint>,
	) -> SCResult<BigUint> {
		Ok(BigUint::from(1u64))

		//todo
	}
}
