use super::{ArwenBigInt, ArwenBigUint};
use crate::ArwenApiImpl;
use elrond_wasm::api::BigUintApi;
use elrond_wasm::api::ContractHookApi;
use elrond_wasm::{Address, ArgBuffer, Box, BoxedBytes, CodeMetadata, Vec, H256};

#[rustfmt::skip]
extern {
    fn getSCAddress(resultOffset: *mut u8);
	fn getOwnerAddress(resultOffset: *mut u8);

	/// Currently not used.
	#[allow(dead_code)]
	fn blockHash(nonce: i64, resultOffset: *mut u8) -> i32;
	
    /// Currently not used.
	#[allow(dead_code)]
    fn getFunction(functionOffset: *const u8) -> i32;

    fn transferValue(dstOffset: *const u8, valueOffset: *const u8, dataOffset: *const u8, length: i32) -> i32;
	fn asyncCall(dstOffset: *const u8, valueOffset: *const u8, dataOffset: *const u8, length: i32);
	fn createContract(gas: u64,
		valueOffset: *const u8,
		codeOffset: *const u8, codeMetadataOffset: *const u8, length: i32,
		resultOffset: *const u8,
		numArguments: i32, argumentsLengthOffset: *const u8, dataOffset: *const u8) -> i32;

	fn getCaller(resultOffset: *mut u8);
	
    /// Currently not used.
	#[allow(dead_code)]
	fn callValue(resultOffset: *const u8) -> i32;

    /// Currently not used.
	#[allow(dead_code)]
	fn getESDTValue(resultOffset: *const u8) -> i32;
	
    fn getESDTTokenName(resultOffset: *const u8) -> i32;

    fn getGasLeft() -> i64;
    fn getBlockTimestamp() -> i64;
    fn getBlockNonce() -> i64;
    fn getBlockRound() -> i64;
    fn getBlockEpoch() -> i64;
    fn getBlockRandomSeed(resultOffset: *mut u8);
    /// Currently not used.
	#[allow(dead_code)]
	fn getStateRootHash(resultOffset: *mut u8);
    fn getPrevBlockTimestamp() -> i64;
    fn getPrevBlockNonce() -> i64;
    fn getPrevBlockRound() -> i64;
    fn getPrevBlockEpoch() -> i64;
    fn getPrevBlockRandomSeed(resultOffset: *const u8);
	fn getOriginalTxHash(resultOffset: *const u8);
	
	fn executeOnDestContext(gas: u64, addressOffset: *const u8, valueOffset: *const u8, 
		functionOffset: *const u8, functionLength: i32, 
		numArguments: i32, argumentsLengthOffset: *const u8, dataOffset: *const u8);
	fn executeOnDestContextByCaller(gas: u64, addressOffset: *const u8, valueOffset: *const u8, 
		functionOffset: *const u8, functionLength: i32, 
		numArguments: i32, argumentsLengthOffset: *const u8, dataOffset: *const u8);
	fn executeOnSameContext(gas: u64, addressOffset: *const u8, valueOffset: *const u8, 
		functionOffset: *const u8, functionLength: i32, 
		numArguments: i32, argumentsLengthOffset: *const u8, dataOffset: *const u8);

    // big int API
    fn bigIntNew(value: i64) -> i32;
	fn bigIntGetExternalBalance(address_ptr: *const u8, dest: i32);
	fn bigIntGetCallValue(dest: i32);
    fn bigIntGetESDTCallValue(dest: i32);
}

impl ContractHookApi<ArwenBigInt, ArwenBigUint> for ArwenApiImpl {
	type Storage = Self;

	fn get_storage_raw(&self) -> Self::Storage {
		self.clone()
	}

	#[inline]
	fn get_sc_address(&self) -> Address {
		unsafe {
			let mut res = Address::zero();
			getSCAddress(res.as_mut_ptr());
			res
		}
	}

	#[inline]
	fn get_owner_address(&self) -> Address {
		unsafe {
			let mut res = Address::zero();
			getOwnerAddress(res.as_mut_ptr());
			res
		}
	}

	#[inline]
	fn get_caller(&self) -> Address {
		unsafe {
			let mut res = Address::zero();
			getCaller(res.as_mut_ptr());
			res
		}
	}

	fn get_balance(&self, address: &Address) -> ArwenBigUint {
		unsafe {
			let result = bigIntNew(0);
			bigIntGetExternalBalance(address.as_ref().as_ptr(), result);
			ArwenBigUint { handle: result }
		}
	}

	#[inline]
	fn get_call_value_big_uint(&self) -> ArwenBigUint {
		unsafe {
			let result = bigIntNew(0);
			bigIntGetCallValue(result);
			ArwenBigUint { handle: result }
		}
	}

	#[inline]
	fn get_esdt_value_big_uint(&self) -> ArwenBigUint {
		unsafe {
			let result = bigIntNew(0);
			bigIntGetESDTCallValue(result);
			ArwenBigUint { handle: result }
		}
	}

	fn get_esdt_token_name(&self) -> Vec<u8> {
		// TODO: returning a boxed slice instead should marginally improve performance
		unsafe {
			let mut name = Vec::with_capacity(32);
			let name_len = getESDTTokenName(name.as_mut_ptr());
			name.set_len(name_len as usize);
			name
		}
	}

	fn send_tx(&self, to: &Address, amount: &ArwenBigUint, data: &[u8]) {
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

	#[inline]
	fn get_tx_hash(&self) -> H256 {
		unsafe {
			let mut res = H256::zero();
			getOriginalTxHash(res.as_mut_ptr());
			res.into()
		}
	}

	#[inline]
	fn get_gas_left(&self) -> u64 {
		unsafe { getGasLeft() as u64 }
	}

	#[inline]
	fn get_block_timestamp(&self) -> u64 {
		unsafe { getBlockTimestamp() as u64 }
	}

	#[inline]
	fn get_block_nonce(&self) -> u64 {
		unsafe { getBlockNonce() as u64 }
	}

	#[inline]
	fn get_block_round(&self) -> u64 {
		unsafe { getBlockRound() as u64 }
	}

	#[inline]
	fn get_block_epoch(&self) -> u64 {
		unsafe { getBlockEpoch() as u64 }
	}

	#[inline]
	fn get_block_random_seed(&self) -> Box<[u8; 48]> {
		unsafe {
			let mut res = [0u8; 48];
			getBlockRandomSeed(res.as_mut_ptr());
			Box::new(res)
		}
	}

	#[inline]
	fn get_prev_block_timestamp(&self) -> u64 {
		unsafe { getPrevBlockTimestamp() as u64 }
	}

	#[inline]
	fn get_prev_block_nonce(&self) -> u64 {
		unsafe { getPrevBlockNonce() as u64 }
	}

	#[inline]
	fn get_prev_block_round(&self) -> u64 {
		unsafe { getPrevBlockRound() as u64 }
	}

	#[inline]
	fn get_prev_block_epoch(&self) -> u64 {
		unsafe { getPrevBlockEpoch() as u64 }
	}

	#[inline]
	fn get_prev_block_random_seed(&self) -> Box<[u8; 48]> {
		unsafe {
			let mut res = [0u8; 48];
			getPrevBlockRandomSeed(res.as_mut_ptr());
			Box::new(res)
		}
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
