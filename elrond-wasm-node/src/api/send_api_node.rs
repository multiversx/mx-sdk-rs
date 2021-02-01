use super::ArwenBigUint;
use crate::ArwenApiImpl;
use elrond_wasm::api::{BigUintApi, SendApi};
use elrond_wasm::types::{Address, TokenIdentifier};

extern "C" {
	fn transferValue(
		dstOffset: *const u8,
		valueOffset: *const u8,
		dataOffset: *const u8,
		length: i32,
	) -> i32;
}

impl SendApi<ArwenBigUint> for ArwenApiImpl {
	fn egld(&self, to: &Address, amount: &ArwenBigUint, data: &[u8]) {
		let amount_bytes32 = amount.to_bytes_be_pad_right(32).unwrap(); // TODO: unwrap panics, remove
		unsafe {
			transferValue(
				to.as_ref().as_ptr(),
				amount_bytes32.as_ptr(),
				data.as_ptr(),
				data.len() as i32,
			);
		}
	}
}
