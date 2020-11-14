use elrond_wasm::{Address, Box, BoxedBytes, Vec, H256};

use crate::big_int::*;
use crate::big_uint::*;
use crate::ext_error;
use elrond_wasm::err_msg;
use elrond_wasm::BigUintApi;
use elrond_wasm::ContractHookApi;

const ADDRESS_LENGTH: usize = 32;
const TOPIC_LENGTH: usize = 32;

#[rustfmt::skip]
extern {
    fn getSCAddress(resultOffset: *mut u8);
    fn getOwnerAddress(resultOffset: *mut u8);
    fn blockHash(nonce: i64, resultOffset: *mut u8) -> i32;
    fn getNumArguments() -> i32;
    fn getArgumentLength(id: i32) -> i32;
    fn getArgument(id: i32, dstOffset: *mut u8) -> i32;
    fn getFunction(functionOffset: *const u8) -> i32;
    fn storageStore(keyOffset: *const u8, keyLength: i32, dataOffset: *const u8, dataLength: i32) -> i32;
    fn storageLoadLength(keyOffset: *const u8, keyLength: i32) -> i32;
    fn storageLoad(keyOffset: *const u8, keyLength: i32, dataOffset: *mut u8) -> i32;

    fn transferValue(dstOffset: *const u8, valueOffset: *const u8, dataOffset: *const u8, length: i32) -> i32;
    fn asyncCall(dstOffset: *const u8, valueOffset: *const u8, dataOffset: *const u8, length: i32);

    fn getCaller(resultOffset: *mut u8);
    fn callValue(resultOffset: *const u8) -> i32;
    fn getESDTValue(resultOffset: *const u8) -> usize;
    fn getESDTTokenName(resultOffset: *const u8) -> usize;
    fn writeLog(pointer: *const u8, length: i32, topicPtr: *const u8, numTopics: i32);
    fn finish(dataOffset: *const u8, length: i32);

    fn getGasLeft() -> i64;
    fn getBlockTimestamp() -> i64;
    fn getBlockNonce() -> i64;
    fn getBlockRound() -> i64;
    fn getBlockEpoch() -> i64;
    fn getBlockRandomSeed(resultOffset: *mut u8);
    fn getStateRootHash(resultOffset: *mut u8);
    fn getPrevBlockTimestamp() -> i64;
    fn getPrevBlockNonce() -> i64;
    fn getPrevBlockRound() -> i64;
    fn getPrevBlockEpoch() -> i64;
    fn getPrevBlockRandomSeed(resultOffset: *const u8);
    fn getOriginalTxHash(resultOffset: *const u8);

    // big int API
    fn bigIntNew(value: i64) -> i32;
    fn bigIntStorageStoreUnsigned(keyOffset: *const u8, keyLength: i32, source: i32) -> i32;
    fn bigIntStorageLoadUnsigned(keyOffset: *const u8, keyLength: i32, destination: i32) -> i32;
    fn bigIntGetExternalBalance(address_ptr: *const u8, dest: i32);
    fn bigIntGetUnsignedArgument(arg_id: i32, dest: i32);
    fn bigIntGetSignedArgument(arg_id: i32, dest: i32);
    fn bigIntGetCallValue(dest: i32);
    fn bigIntGetESDTCallValue(dest: i32);
    fn bigIntFinishUnsigned(bih: i32);
    fn bigIntFinishSigned(bih: i32);

    // small int API
    fn smallIntGetUnsignedArgument(id: i32) -> i64;
    fn smallIntGetSignedArgument(id: i32) -> i64;
    fn smallIntFinishUnsigned(value: i64);
    fn smallIntFinishSigned(value: i64);
    fn smallIntStorageStoreUnsigned(keyOffset: *const u8, keyLength: i32, value: i64) -> i32;
    fn smallIntStorageStoreSigned(keyOffset: *const u8, keyLength: i32, value: i64) -> i32;
    fn smallIntStorageLoadUnsigned(keyOffset: *const u8, keyLength: i32) -> i64;
    fn smallIntStorageLoadSigned(keyOffset: *const u8, keyLength: i32) -> i64;
    
    // crypto API
    fn sha256(dataOffset: *const u8, length: i32, resultOffset: *mut u8) -> i32;
    fn keccak256(dataOffset: *const u8, length: i32, resultOffset: *mut u8) -> i32;
}

pub struct ArwenApiImpl {}
impl elrond_wasm::ContractHookApi<ArwenBigInt, ArwenBigUint> for ArwenApiImpl {
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

	fn storage_store_slice_u8(&self, key: &[u8], value: &[u8]) {
		unsafe {
			storageStore(
				key.as_ref().as_ptr(),
				key.len() as i32,
				value.as_ptr(),
				value.len() as i32,
			);
		}
	}

	#[inline]
	fn storage_load_len(&self, key: &[u8]) -> usize {
		unsafe { storageLoadLength(key.as_ref().as_ptr(), key.len() as i32) as usize }
	}

	fn storage_load_vec_u8(&self, key: &[u8]) -> Vec<u8> {
		unsafe {
			let value_len = self.storage_load_len(key);
			let mut res = Vec::with_capacity(value_len);
			storageLoad(key.as_ref().as_ptr(), key.len() as i32, res.as_mut_ptr());
			res.set_len(value_len);
			res
		}
	}

	fn storage_load_boxed_bytes(&self, key: &[u8]) -> BoxedBytes {
		let len = self.storage_load_len(key);
		unsafe {
			let mut res = BoxedBytes::allocate(len);
			if len > 0 {
				storageLoad(key.as_ref().as_ptr(), key.len() as i32, res.as_mut_ptr());
			}
			res
		}
	}

	#[inline]
	fn storage_store_bytes32(&self, key: &[u8], value: &[u8; 32]) {
		unsafe {
			storageStore(key.as_ref().as_ptr(), key.len() as i32, value.as_ptr(), 32);
		}
	}

	fn storage_load_bytes32(&self, key: &[u8]) -> [u8; 32] {
		unsafe {
			let mut res = [0u8; 32];
			let len = storageLoad(key.as_ref().as_ptr(), key.len() as i32, res.as_mut_ptr());
			if len != 32 {
				ext_error::signal_error(err_msg::STORAGE_NOT_32_BYTES);
			}
			res
		}
	}

	#[inline]
	fn storage_store_big_uint(&self, key: &[u8], value: &ArwenBigUint) {
		unsafe {
			bigIntStorageStoreUnsigned(key.as_ref().as_ptr(), key.len() as i32, value.handle);
		}
	}

	#[inline]
	fn storage_load_big_uint(&self, key: &[u8]) -> ArwenBigUint {
		unsafe {
			let handle = bigIntNew(0);
			bigIntStorageLoadUnsigned(key.as_ref().as_ptr(), key.len() as i32, handle);
			ArwenBigUint { handle }
		}
	}

	#[inline]
	fn storage_store_big_uint_raw(&self, key: &[u8], handle: i32) {
		unsafe {
			bigIntStorageStoreUnsigned(key.as_ref().as_ptr(), key.len() as i32, handle);
		}
	}

	#[inline]
	fn storage_load_big_uint_raw(&self, key: &[u8]) -> i32 {
		unsafe {
			let handle = bigIntNew(0);
			bigIntStorageLoadUnsigned(key.as_ref().as_ptr(), key.len() as i32, handle);
			handle
		}
	}

	#[inline]
	fn storage_store_big_int(&self, key: &[u8], value: &ArwenBigInt) {
		unsafe {
			// TODO: convert to 2's complement
			bigIntStorageStoreUnsigned(key.as_ref().as_ptr(), key.len() as i32, value.handle);
		}
	}

	#[inline]
	fn storage_load_big_int(&self, key: &[u8]) -> ArwenBigInt {
		unsafe {
			let result = bigIntNew(0);
			// TODO: convert from 2's complement
			bigIntStorageLoadUnsigned(key.as_ref().as_ptr(), key.len() as i32, result);
			ArwenBigInt { handle: result }
		}
	}

	#[cfg(feature = "small-int-ei")]
	#[inline]
	fn storage_store_u64(&self, key: &[u8], value: u64) {
		unsafe {
			smallIntStorageStoreUnsigned(key.as_ref().as_ptr(), key.len() as i32, value as i64);
		}
	}

	#[cfg(not(feature = "small-int-ei"))]
	#[inline]
	fn storage_store_u64(&self, key: &[u8], value: u64) {
		let mut buffer = Vec::<u8>::new();
		elrond_wasm::elrond_codec::top_encode_number_to_output(&mut buffer, value, false);
		self.storage_store_slice_u8(key, &buffer[..]);
	}

	#[cfg(feature = "small-int-ei")]
	#[inline]
	fn storage_store_i64(&self, key: &[u8], value: i64) {
		unsafe {
			smallIntStorageStoreSigned(key.as_ref().as_ptr(), key.len() as i32, value);
		}
	}

	#[cfg(not(feature = "small-int-ei"))]
	#[inline]
	fn storage_store_i64(&self, key: &[u8], value: i64) {
		let mut buffer = Vec::<u8>::new();
		elrond_wasm::elrond_codec::top_encode_number_to_output(&mut buffer, value as u64, true);
		self.storage_store_slice_u8(key, &buffer[..]);
	}

	#[cfg(feature = "small-int-ei")]
	#[inline]
	fn storage_load_u64(&self, key: &[u8]) -> u64 {
		unsafe { smallIntStorageLoadUnsigned(key.as_ref().as_ptr(), key.len() as i32) as u64 }
	}

	#[cfg(not(feature = "small-int-ei"))]
	#[inline(never)]
	fn storage_load_u64(&self, key: &[u8]) -> u64 {
		let bytes = self.storage_load_boxed_bytes(key);
		if bytes.len() > 8 {
			ext_error::signal_error(err_msg::STORAGE_VALUE_OUT_OF_RANGE);
		}
		elrond_wasm::elrond_codec::bytes_to_number(&bytes.into_box(), false)
	}

	#[cfg(feature = "small-int-ei")]
	#[inline]
	fn storage_load_i64(&self, key: &[u8]) -> i64 {
		unsafe { smallIntStorageLoadSigned(key.as_ref().as_ptr(), key.len() as i32) }
	}

	#[cfg(not(feature = "small-int-ei"))]
	#[inline(never)]
	fn storage_load_i64(&self, key: &[u8]) -> i64 {
		let bytes = self.storage_load_boxed_bytes(key);
		if bytes.len() > 8 {
			ext_error::signal_error(err_msg::STORAGE_VALUE_OUT_OF_RANGE);
		}
		elrond_wasm::elrond_codec::bytes_to_number(&bytes.into_box(), true) as i64
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

	fn send_tx(&self, to: &Address, amount: &ArwenBigUint, message: &str) {
		let amount_bytes32 = amount.to_bytes_be_pad_right(32).unwrap(); // TODO: unwrap panics, remove
		unsafe {
			transferValue(
				to.as_ref().as_ptr(),
				amount_bytes32.as_ptr(),
				message.as_ptr(),
				message.len() as i32,
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

	fn sha256(&self, data: &[u8]) -> H256 {
		unsafe {
			let mut res = H256::zero();
			sha256(data.as_ptr(), data.len() as i32, res.as_mut_ptr());
			res
		}
	}

	fn keccak256(&self, data: &[u8]) -> H256 {
		unsafe {
			let mut res = H256::zero();
			keccak256(data.as_ptr(), data.len() as i32, res.as_mut_ptr());
			res
		}
	}
}

impl elrond_wasm::ContractIOApi<ArwenBigInt, ArwenBigUint> for ArwenApiImpl {
	#[inline]
	fn get_num_arguments(&self) -> i32 {
		unsafe { getNumArguments() }
	}

	fn check_not_payable(&self) {
		if self.get_call_value_big_uint() > 0 {
			self.signal_error(err_msg::NON_PAYABLE);
		}
	}

	#[inline]
	fn get_argument_len(&self, arg_index: i32) -> usize {
		unsafe { getArgumentLength(arg_index) as usize }
	}

	fn copy_argument_to_slice(&self, arg_index: i32, slice: &mut [u8]) {
		unsafe {
			let byte_len = getArgument(arg_index, slice.as_mut_ptr()) as usize;
			if byte_len != slice.len() {
				self.signal_error(err_msg::ARG_BAD_LENGTH);
			}
		}
	}

	fn get_argument_vec_u8(&self, arg_index: i32) -> Vec<u8> {
		let len = self.get_argument_len(arg_index);
		let mut res = Vec::with_capacity(len);
		if len > 0 {
			unsafe {
				res.set_len(len);
				getArgument(arg_index, res.as_mut_ptr());
			}
		}
		res
	}

	fn get_argument_boxed_bytes(&self, arg_index: i32) -> BoxedBytes {
		let len = self.get_argument_len(arg_index);
		unsafe {
			let mut res = BoxedBytes::allocate(len);
			if len > 0 {
				getArgument(arg_index, res.as_mut_ptr());
			}
			res
		}
	}

	#[inline]
	fn get_argument_big_uint(&self, arg_id: i32) -> ArwenBigUint {
		ArwenBigUint {
			handle: self.get_argument_big_uint_raw(arg_id),
		}
	}

	#[inline]
	fn get_argument_big_int(&self, arg_id: i32) -> ArwenBigInt {
		ArwenBigInt {
			handle: self.get_argument_big_int_raw(arg_id),
		}
	}

	fn get_argument_big_uint_raw(&self, arg_id: i32) -> i32 {
		unsafe {
			let handle = bigIntNew(0);
			bigIntGetUnsignedArgument(arg_id, handle);
			handle
		}
	}

	fn get_argument_big_int_raw(&self, arg_id: i32) -> i32 {
		unsafe {
			let handle = bigIntNew(0);
			bigIntGetSignedArgument(arg_id, handle);
			handle
		}
	}

	#[cfg(feature = "small-int-ei")]
	#[inline]
	fn get_argument_u64(&self, arg_id: i32) -> u64 {
		unsafe { smallIntGetUnsignedArgument(arg_id) as u64 }
	}

	#[cfg(not(feature = "small-int-ei"))]
	fn get_argument_u64(&self, arg_id: i32) -> u64 {
		let bytes = self.get_argument_boxed_bytes(arg_id);
		if bytes.len() > 8 {
			ext_error::signal_error(err_msg::ARG_OUT_OF_RANGE);
		}
		elrond_wasm::elrond_codec::bytes_to_number(&bytes.into_box(), false)
	}

	#[cfg(feature = "small-int-ei")]
	#[inline]
	fn get_argument_i64(&self, arg_id: i32) -> i64 {
		unsafe { smallIntGetSignedArgument(arg_id) }
	}

	#[cfg(not(feature = "small-int-ei"))]
	fn get_argument_i64(&self, arg_id: i32) -> i64 {
		let bytes = self.get_argument_boxed_bytes(arg_id);
		if bytes.len() > 8 {
			ext_error::signal_error(err_msg::ARG_OUT_OF_RANGE);
		}
		elrond_wasm::elrond_codec::bytes_to_number(&bytes.into_box(), true) as i64
	}

	#[inline]
	fn finish_slice_u8(&self, slice: &[u8]) {
		unsafe {
			finish(slice.as_ptr(), slice.len() as i32);
		}
	}

	#[inline]
	fn finish_big_int(&self, b: &ArwenBigInt) {
		unsafe {
			bigIntFinishSigned(b.handle);
		}
	}

	#[inline]
	fn finish_big_uint(&self, b: &ArwenBigUint) {
		unsafe {
			bigIntFinishUnsigned(b.handle);
		}
	}

	#[inline]
	fn finish_big_int_raw(&self, handle: i32) {
		unsafe {
			bigIntFinishSigned(handle);
		}
	}

	#[inline]
	fn finish_big_uint_raw(&self, handle: i32) {
		unsafe {
			bigIntFinishUnsigned(handle);
		}
	}

	#[cfg(feature = "small-int-ei")]
	#[inline]
	fn finish_u64(&self, value: u64) {
		unsafe {
			smallIntFinishUnsigned(value as i64);
		}
	}

	#[cfg(not(feature = "small-int-ei"))]
	fn finish_u64(&self, value: u64) {
		let mut buffer = Vec::<u8>::new();
		elrond_wasm::elrond_codec::top_encode_number_to_output(&mut buffer, value, false);
		self.finish_slice_u8(&buffer[..]);
	}

	#[cfg(feature = "small-int-ei")]
	#[inline]
	fn finish_i64(&self, value: i64) {
		unsafe {
			smallIntFinishSigned(value);
		}
	}

	#[cfg(not(feature = "small-int-ei"))]
	fn finish_i64(&self, value: i64) {
		let mut buffer = Vec::<u8>::new();
		elrond_wasm::elrond_codec::top_encode_number_to_output(&mut buffer, value as u64, true);
		self.finish_slice_u8(&buffer[..]);
	}

	#[inline]
	fn signal_error(&self, message: &[u8]) -> ! {
		ext_error::signal_error(message)
	}

	fn write_log(&self, topics: &[[u8; 32]], data: &[u8]) {
		let mut topics_raw = [0u8; TOPIC_LENGTH * 10]; // hopefully we never have more than 10 topics
		for i in 0..topics.len() {
			topics_raw[TOPIC_LENGTH * i..TOPIC_LENGTH * (i + 1)].copy_from_slice(&topics[i]);
		}
		unsafe {
			writeLog(
				data.as_ptr(),
				data.len() as i32,
				topics_raw.as_ptr(),
				topics.len() as i32,
			);
		}
	}
}

/// Should be no-op. The API implementation is zero-sized.
impl Clone for ArwenApiImpl {
	#[inline]
	fn clone(&self) -> Self {
		ArwenApiImpl {}
	}
}
