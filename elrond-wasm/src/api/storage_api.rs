use crate::types::BoxedBytes;
use alloc::vec::Vec;

pub trait StorageReadApi {
	fn storage_load_len(&self, key: &[u8]) -> usize;

	fn storage_load_vec_u8(&self, key: &[u8]) -> Vec<u8>;

	fn storage_load_boxed_bytes(&self, key: &[u8]) -> BoxedBytes {
		self.storage_load_vec_u8(key).into()
	}

	fn storage_load_big_uint_raw(&self, key: &[u8]) -> i32;

	fn storage_load_u64(&self, key: &[u8]) -> u64;

	fn storage_load_i64(&self, key: &[u8]) -> i64;
}

pub trait StorageWriteApi {
	fn storage_store_slice_u8(&self, key: &[u8], value: &[u8]);

	fn storage_store_big_uint_raw(&self, key: &[u8], handle: i32);

	fn storage_store_u64(&self, key: &[u8], value: u64);

	fn storage_store_i64(&self, key: &[u8], value: i64);
}
