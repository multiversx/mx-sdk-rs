use crate::api::{StorageReadApi, StorageReadApiImpl, StorageWriteApi, StorageWriteApiImpl};
use alloc::boxed::Box;

use super::UncallableApi;

impl StorageReadApi for UncallableApi {
    type StorageReadApiImpl = Self;

    fn storage_read_api_impl() -> Self {
        unreachable!()
    }
}

impl StorageReadApiImpl for UncallableApi {
    fn storage_load_len(&self, _key: &[u8]) -> usize {
        unreachable!()
    }

    fn storage_load_to_heap(&self, _key: &[u8]) -> Box<[u8]> {
        unreachable!()
    }

    fn storage_load_big_uint_raw(&self, _key: &[u8], _dest: i32) {
        unreachable!()
    }

    fn storage_load_managed_buffer_raw(&self, _key_handle: i32, _dest: i32) {
        unreachable!()
    }

    fn storage_load_from_address(&self, _address_handle: i32, _key_handle: i32, _dest: i32) {
        unreachable!()
    }
}

impl StorageWriteApi for UncallableApi {
    type StorageWriteApiImpl = UncallableApi;

    fn storage_write_api_impl() -> Self::StorageWriteApiImpl {
        unreachable!()
    }
}

impl StorageWriteApiImpl for super::UncallableApi {
    fn storage_store_slice_u8(&self, _key: &[u8], _value: &[u8]) {
        unreachable!()
    }

    fn storage_store_big_uint_raw(&self, _key: &[u8], _value_handle: Self::BigIntHandle) {
        unreachable!()
    }

    fn storage_store_managed_buffer_raw(
        &self,
        _key_handle: Self::ManagedBufferHandle,
        _value_handle: Self::ManagedBufferHandle,
    ) {
        unreachable!()
    }

    fn storage_store_managed_buffer_clear(&self, _key_handle: Self::ManagedBufferHandle) {
        unreachable!()
    }
}
