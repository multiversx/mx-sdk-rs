use super::ArwenBigUint;
use crate::ArwenApiImpl;
use elrond_wasm::api::{BigUintApi, SendApi};
use elrond_wasm::types::{Address, TokenIdentifier};

extern "C" {
	fn transferValue(
		dstOffset: *const u8,
		valueOffset: *const u8,
		dataOffset: *const u8,
		dataLength: i32,
	) -> i32;

	fn transferESDT(
		dstOffset: *const u8,
		tokenIdOffset: *const u8,
		tokenIdLen: i32,
		valueOffset: *const u8,
		gasLimit: i64,
		dataOffset: *const u8,
		dataLength: i32,
	) -> i32;
}

impl SendApi<ArwenBigUint> for ArwenApiImpl {
	fn direct_egld(&self, to: &Address, amount: &ArwenBigUint, data: &[u8]) {
		let amount_bytes32 = amount.to_bytes_be_pad_right(32).unwrap(); // TODO: unwrap panics, remove
		unsafe {
			let _ = transferValue(
				to.as_ref().as_ptr(),
				amount_bytes32.as_ptr(),
				data.as_ptr(),
				data.len() as i32,
			);
		}
	}

	fn direct_esdt_explicit_gas(&self, to: &Address, token: &[u8], amount: &ArwenBigUint, gas_limit: u64, data: &[u8]) {
		let amount_bytes32 = amount.to_bytes_be_pad_right(32).unwrap(); // TODO: unwrap panics, remove
		unsafe {
			let _ = transferESDT(
				to.as_ref().as_ptr(),
				token.as_ptr(),
				token.len() as i32,
				amount_bytes32.as_ptr(),
				gas_limit as i64,
				data.as_ptr(),
				data.len() as i32,
			);
		}
	}
}
