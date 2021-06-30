elrond_wasm::imports!();

/// TODO: add neg, abs, sqrt etc. here
#[elrond_wasm_derive::module]
pub trait BigIntMethods {
	#[endpoint]
	fn big_uint_to_u64(&self, bu: &Self::BigUint) -> OptionalResult<u64> {
		bu.to_u64().into()
	}

	#[endpoint]
	fn big_int_to_i64(&self, bi: &Self::BigInt) -> OptionalResult<i64> {
		bi.to_i64().into()
	}
}
