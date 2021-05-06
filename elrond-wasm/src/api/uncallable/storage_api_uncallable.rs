use crate::api::{ErrorApi, StorageReadApi, StorageWriteApi};
use alloc::vec::Vec;

/// Dummy type with no implementation.
/// Provides context in ABI generators.
#[derive(Clone)]
pub struct StorageApiUncallable;

impl StorageReadApi for StorageApiUncallable {
	fn storage_load_len(&self, _key: &[u8]) -> usize {
		unreachable!()
	}

	fn storage_load_vec_u8(&self, _key: &[u8]) -> Vec<u8> {
		unreachable!()
	}

	fn storage_load_big_uint_raw(&self, _key: &[u8]) -> i32 {
		unreachable!()
	}

	fn storage_load_u64(&self, _key: &[u8]) -> u64 {
		unreachable!()
	}

	fn storage_load_i64(&self, _key: &[u8]) -> i64 {
		unreachable!()
	}
}

impl StorageWriteApi for StorageApiUncallable {
	fn storage_store_slice_u8(&self, _key: &[u8], _value: &[u8]) {
		unreachable!()
	}

	fn storage_store_big_uint_raw(&self, _key: &[u8], _handle: i32) {
		unreachable!()
	}

	fn storage_store_u64(&self, _key: &[u8], _value: u64) {
		unreachable!()
	}

	fn storage_store_i64(&self, _key: &[u8], _value: i64) {
		unreachable!()
	}
}

impl ErrorApi for StorageApiUncallable {
	fn signal_error(&self, _message: &[u8]) -> ! {
		unreachable!()
	}
}
