use super::ArwenBigUint;
use crate::ArwenApiImpl;
use elrond_wasm::api::{BigUintApi, SendApi};
use elrond_wasm::types::{Address, ArgBuffer, BoxedBytes, CodeMetadata};

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

	fn asyncCall(dstOffset: *const u8, valueOffset: *const u8, dataOffset: *const u8, length: i32);

	fn createContract(
		gas: u64,
		valueOffset: *const u8,
		codeOffset: *const u8,
		codeMetadataOffset: *const u8,
		length: i32,
		resultOffset: *const u8,
		numArguments: i32,
		argumentsLengthOffset: *const u8,
		dataOffset: *const u8,
	) -> i32;

	fn executeOnDestContext(
		gas: u64,
		addressOffset: *const u8,
		valueOffset: *const u8,
		functionOffset: *const u8,
		functionLength: i32,
		numArguments: i32,
		argumentsLengthOffset: *const u8,
		dataOffset: *const u8,
	);
	fn executeOnDestContextByCaller(
		gas: u64,
		addressOffset: *const u8,
		valueOffset: *const u8,
		functionOffset: *const u8,
		functionLength: i32,
		numArguments: i32,
		argumentsLengthOffset: *const u8,
		dataOffset: *const u8,
	);
	fn executeOnSameContext(
		gas: u64,
		addressOffset: *const u8,
		valueOffset: *const u8,
		functionOffset: *const u8,
		functionLength: i32,
		numArguments: i32,
		argumentsLengthOffset: *const u8,
		dataOffset: *const u8,
	);
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

	fn direct_esdt_explicit_gas(
		&self,
		to: &Address,
		token: &[u8],
		amount: &ArwenBigUint,
		gas_limit: u64,
		data: &[u8],
	) {
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

	fn async_call(&self, to: &Address, amount: &ArwenBigUint, data: &[u8]) {
		let amount_bytes32 = amount.to_bytes_be_pad_right(32).unwrap(); // TODO: unwrap panics, remove
		unsafe {
			asyncCall(
				to.as_ref().as_ptr(),
				amount_bytes32.as_ptr(),
				data.as_ptr(),
				data.len() as i32,
			);
		}
	}

	fn deploy_contract(
		&self,
		gas: u64,
		amount: &ArwenBigUint,
		code: &BoxedBytes,
		code_metadata: CodeMetadata,
		arg_buffer: &ArgBuffer,
	) -> Address {
		let amount_bytes32 = amount.to_bytes_be_pad_right(32).unwrap(); // TODO: unwrap panics, remove
		let mut new_address = Address::zero();
		unsafe {
			let _ = createContract(
				gas,
				amount_bytes32.as_ptr(),
				code.as_ptr(),
				code_metadata.as_ptr(),
				code.len() as i32,
				new_address.as_mut_ptr(),
				arg_buffer.num_args() as i32,
				arg_buffer.arg_lengths_bytes_ptr(),
				arg_buffer.arg_data_ptr(),
			);
		}
		new_address
	}

	fn execute_on_dest_context(
		&self,
		gas: u64,
		address: &Address,
		value: &ArwenBigUint,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) {
		unsafe {
			let value_bytes32 = value.to_bytes_be_pad_right(32).unwrap(); // TODO: unwrap panics, remove

			executeOnDestContext(
				gas,
				address.as_ref().as_ptr(),
				value_bytes32.as_ptr(),
				function.as_ptr(),
				function.len() as i32,
				arg_buffer.num_args() as i32,
				arg_buffer.arg_lengths_bytes_ptr(),
				arg_buffer.arg_data_ptr(),
			);
		}
	}

	fn execute_on_dest_context_by_caller(
		&self,
		gas: u64,
		address: &Address,
		value: &ArwenBigUint,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) {
		unsafe {
			let value_bytes32 = value.to_bytes_be_pad_right(32).unwrap(); // TODO: unwrap panics, remove

			executeOnDestContextByCaller(
				gas,
				address.as_ref().as_ptr(),
				value_bytes32.as_ptr(),
				function.as_ptr(),
				function.len() as i32,
				arg_buffer.num_args() as i32,
				arg_buffer.arg_lengths_bytes_ptr(),
				arg_buffer.arg_data_ptr(),
			);
		}
	}

	fn execute_on_same_context(
		&self,
		gas: u64,
		address: &Address,
		value: &ArwenBigUint,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) {
		unsafe {
			let value_bytes32 = value.to_bytes_be_pad_right(32).unwrap(); // TODO: unwrap panics, remove

			executeOnSameContext(
				gas,
				address.as_ref().as_ptr(),
				value_bytes32.as_ptr(),
				function.as_ptr(),
				function.len() as i32,
				arg_buffer.num_args() as i32,
				arg_buffer.arg_lengths_bytes_ptr(),
				arg_buffer.arg_data_ptr(),
			);
		}
	}
}
