elrond_wasm::imports!();

#[elrond_wasm_derive::module]
pub trait BigIntMethods {
	#[endpoint]
	fn sqrt_big_uint(&self, a: Self::BigUint) -> Self::BigUint {
		a.sqrt()
	}

	#[endpoint]
	fn sqrt_big_uint_ref(&self, a: &Self::BigUint) -> Self::BigUint {
		a.sqrt()
	}

	#[endpoint]
	fn log2_big_uint(&self, a: Self::BigUint) -> u32 {
		a.log2()
	}

	#[endpoint]
	fn log2_big_uint_ref(&self, a: &Self::BigUint) -> u32 {
		a.log2()
	}

	#[endpoint]
	fn pow_big_int(&self, a: Self::BigInt, b: u32) -> Self::BigInt {
		a.pow(b)
	}

	#[endpoint]
	fn pow_big_int_ref(&self, a: &Self::BigInt, b: u32) -> Self::BigInt {
		a.pow(b)
	}

	#[endpoint]
	fn pow_big_uint(&self, a: Self::BigUint, b: u32) -> Self::BigUint {
		a.pow(b)
	}

	#[endpoint]
	fn pow_big_uint_ref(&self, a: &Self::BigUint, b: u32) -> Self::BigUint {
		a.pow(b)
	}
}
