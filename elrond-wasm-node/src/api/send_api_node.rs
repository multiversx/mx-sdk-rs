use super::ArwenBigUint;
use crate::ArwenApiImpl;
use elrond_wasm::api::{ContractHookApi, SendApi, StorageReadApi, StorageWriteApi};
use elrond_wasm::types::{Address, ArgBuffer, BoxedBytes, CodeMetadata};

extern "C" {
	fn transferValue(
		dstOffset: *const u8,
		valueOffset: *const u8,
		dataOffset: *const u8,
		dataLength: i32,
	) -> i32;

	fn transferESDTExecute(
		dstOffset: *const u8,
		tokenIdOffset: *const u8,
		tokenIdLen: i32,
		valueOffset: *const u8,
		gasLimit: i64,
		functionOffset: *const u8,
		functionLength: i32,
		numArguments: i32,
		argumentsLengthOffset: *const u8,
		dataOffset: *const u8,
	) -> i32;

	fn asyncCall(
		dstOffset: *const u8,
		valueOffset: *const u8,
		dataOffset: *const u8,
		length: i32,
	) -> !;

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
		unsafe {
			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			let _ = transferValue(
				to.as_ref().as_ptr(),
				amount_bytes32_ptr,
				data.as_ptr(),
				data.len() as i32,
			);
		}
	}

	/// Same as the implementation in the trait, but avoids creating a new ArgBuffer instance.
	fn direct_esdt(&self, to: &Address, token: &[u8], amount: &ArwenBigUint, data: &[u8]) {
		let function = data;
		unsafe {
			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			let _ = transferESDTExecute(
				to.as_ref().as_ptr(),
				token.as_ptr(),
				token.len() as i32,
				amount_bytes32_ptr,
				0i64,
				function.as_ptr(),
				function.len() as i32,
				0i32,
				core::ptr::null(),
				core::ptr::null(),
			);
		}
	}

	fn direct_esdt_execute(
		&self,
		to: &Address,
		token: &[u8],
		amount: &ArwenBigUint,
		gas_limit: u64,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) {
		unsafe {
			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			let _ = transferESDTExecute(
				to.as_ref().as_ptr(),
				token.as_ptr(),
				token.len() as i32,
				amount_bytes32_ptr,
				gas_limit as i64,
				function.as_ptr(),
				function.len() as i32,
				arg_buffer.num_args() as i32,
				arg_buffer.arg_lengths_bytes_ptr(),
				arg_buffer.arg_data_ptr(),
			);
		}
	}

	fn async_call_raw(&self, to: &Address, amount: &ArwenBigUint, data: &[u8]) -> ! {
		unsafe {
			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			asyncCall(
				to.as_ref().as_ptr(),
				amount_bytes32_ptr,
				data.as_ptr(),
				data.len() as i32,
			)
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
		let mut new_address = Address::zero();
		unsafe {
			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			let _ = createContract(
				gas,
				amount_bytes32_ptr,
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
		amount: &ArwenBigUint,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) {
		unsafe {
			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			executeOnDestContext(
				gas,
				address.as_ref().as_ptr(),
				amount_bytes32_ptr,
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
		amount: &ArwenBigUint,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) {
		unsafe {
			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			executeOnDestContextByCaller(
				gas,
				address.as_ref().as_ptr(),
				amount_bytes32_ptr,
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
		amount: &ArwenBigUint,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) {
		unsafe {
			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			executeOnSameContext(
				gas,
				address.as_ref().as_ptr(),
				amount_bytes32_ptr,
				function.as_ptr(),
				function.len() as i32,
				arg_buffer.num_args() as i32,
				arg_buffer.arg_lengths_bytes_ptr(),
				arg_buffer.arg_data_ptr(),
			);
		}
	}

	fn storage_store_tx_hash_key(&self, data: &[u8]) {
		let tx_hash = self.get_tx_hash();
		self.storage_store_slice_u8(tx_hash.as_bytes(), data);
	}

	fn storage_load_tx_hash_key(&self) -> BoxedBytes {
		let tx_hash = self.get_tx_hash();
		self.storage_load_boxed_bytes(tx_hash.as_bytes())
	}
}
