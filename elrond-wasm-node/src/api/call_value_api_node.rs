use crate::ArwenApiImpl;
use elrond_wasm::api::CallValueApi;
use elrond_wasm::types::BoxedBytes;
use super::ArwenBigUint;

const MAX_POSSIBLE_TOKEN_NAME_LENGTH: usize = 32;

extern "C" {
	fn checkNoPayment();

	fn bigIntNew(value: i64) -> i32;
	fn bigIntGetCallValue(dest: i32);
	fn bigIntGetESDTCallValue(dest: i32);
	fn getESDTTokenName(resultOffset: *const u8) -> i32;
}

impl CallValueApi<ArwenBigUint> for ArwenApiImpl {

	#[inline]
	fn check_not_payable(&self) {
		unsafe {
			checkNoPayment();
		}
	}

	fn get_call_value_big_uint(&self) -> ArwenBigUint {
		unsafe {
			let result = bigIntNew(0);
			bigIntGetCallValue(result);
			ArwenBigUint { handle: result }
		}
	}

	fn get_esdt_value_big_uint(&self) -> ArwenBigUint {
		unsafe {
			let result = bigIntNew(0);
			bigIntGetESDTCallValue(result);
			ArwenBigUint { handle: result }
		}
	}

	fn get_esdt_token_name(&self) -> BoxedBytes {
		unsafe {
			let mut buffer = [0u8; MAX_POSSIBLE_TOKEN_NAME_LENGTH];
			let name_len = getESDTTokenName(buffer.as_mut_ptr());
			BoxedBytes::from(&buffer[..name_len as usize])
		}
	}
}