#![no_std]

elrond_wasm::imports!();

#[elrond_wasm_derive::contract(FactorialImpl)]
pub trait Factorial {
	#[init]
	fn init(&self) {}

	#[endpoint]
	fn factorial(&self, value: Self::BigUint) -> Self::BigUint {
		if value == 0 {
			return Self::BigUint::from(1u32);
		}

		let mut result = Self::BigUint::from(1u32);
		let one = Self::BigUint::from(1u32);
		let mut x = Self::BigUint::from(1u32);
		while x <= value {
			result *= &x;
			x += &one;
		}

		result
	}
}
