elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use core::marker::PhantomData;

use crate::curve_function::{CurveArguments, CurveFunction};

pub trait CFType<BigUint> = Fn(BigUint, BigUint, &CurveArguments<BigUint>) -> SCResult<BigUint>;

pub struct CustomFunction<F, BigUint>
where
	BigUint: BigUintApi,
	F: CFType<BigUint>,
{
	pub thingy: F,
	pub phantom: PhantomData<BigUint>,
}

impl<F, BigUint> CurveFunction<BigUint> for CustomFunction<F, BigUint>
where
	for<'a, 'b> &'a BigUint: core::ops::Add<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: core::ops::Sub<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: core::ops::Mul<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: core::ops::Div<&'b BigUint, Output = BigUint>,
	for<'b> BigUint: core::ops::AddAssign<&'b BigUint>,
	for<'b> BigUint: core::ops::SubAssign<&'b BigUint>,
	for<'b> BigUint: core::ops::MulAssign<&'b BigUint>,
	for<'b> BigUint: core::ops::DivAssign<&'b BigUint>,
	F: CFType<BigUint>,
	BigUint: BigUintApi,
{
	fn function(
		&self,
		token_start: BigUint,
		amount: BigUint,
		arguments: &CurveArguments<BigUint>,
	) -> SCResult<BigUint> {
		(self.thingy)(token_start, amount, arguments)
	}
}
