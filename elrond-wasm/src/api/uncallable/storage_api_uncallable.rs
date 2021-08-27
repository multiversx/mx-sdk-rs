use crate::api::{Handle, StorageReadApi, StorageWriteApi};
use alloc::vec::Vec;

impl StorageReadApi for super::UncallableApi {
    fn storage_load_len(&self, _key: &[u8]) -> usize {
        unreachable!()
    }

    fn storage_load_vec_u8(&self, _key: &[u8]) -> Vec<u8> {
        unreachable!()
    }

    fn storage_load_big_uint_raw(&self, _key: &[u8]) -> Handle {
        unreachable!()
    }

    fn storage_load_managed_buffer_raw(&self, _key_handle: Handle) -> Handle {
        unreachable!()
    }

    fn storage_load_managed_buffer_len(&self, _key_handle: Handle) -> usize {
        unreachable!()
    }

    fn storage_load_u64(&self, _key: &[u8]) -> u64 {
        unreachable!()
    }

    fn storage_load_i64(&self, _key: &[u8]) -> i64 {
        unreachable!()
    }
}

impl StorageWriteApi for super::UncallableApi {
    fn storage_store_slice_u8(&self, _key: &[u8], _value: &[u8]) {
        unreachable!()
    }

    fn storage_store_big_uint_raw(&self, _key: &[u8], _value_handle: Handle) {
        unreachable!()
    }

    fn storage_store_managed_buffer_raw(&self, _key_handle: Handle, _value_handle: Handle) {
        unreachable!()
    }

    fn storage_store_managed_buffer_clear(&self, _key_handle: Handle) {
        unreachable!()
    }

    fn storage_store_u64(&self, _key: &[u8], _value: u64) {
        unreachable!()
    }

    fn storage_store_i64(&self, _key: &[u8], _value: i64) {
        unreachable!()
    }
}
