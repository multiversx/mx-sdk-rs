use multiversx_sc::api::{
    StorageReadApi, StorageReadApiImpl, StorageWriteApi, StorageWriteApiImpl,
};

use super::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> StorageReadApi for VMHooksApi<BACKEND_TYPE> {
    type StorageReadApiImpl = Self;

    fn storage_read_api_impl() -> Self::StorageReadApiImpl {
        Self::api_impl()
    }
}

impl<const BACKEND_TYPE: VMHooksBackendType> StorageReadApiImpl for VMHooksApi<BACKEND_TYPE> {
    fn storage_load_len(&self, _key: &[u8]) -> usize {
        todo!()
    }

    fn storage_load_to_heap(&self, _key: &[u8]) -> Box<[u8]> {
        todo!()
    }

    fn storage_load_big_uint_raw(&self, _key: &[u8], _dest: i32) {
        todo!()
    }

    fn storage_load_managed_buffer_raw(&self, _key_handle: i32, _dest: i32) {
        todo!()
    }

    fn storage_load_from_address(&self, _address_handle: i32, _key_handle: i32, _dest: i32) {
        todo!()
    }
}

impl<const BACKEND_TYPE: VMHooksBackendType> StorageWriteApi for VMHooksApi<BACKEND_TYPE> {
    type StorageWriteApiImpl = Self;

    fn storage_write_api_impl() -> Self::StorageWriteApiImpl {
        Self::api_impl()
    }
}

impl<const BACKEND_TYPE: VMHooksBackendType> StorageWriteApiImpl for VMHooksApi<BACKEND_TYPE> {
    fn storage_store_slice_u8(&self, _key: &[u8], _value: &[u8]) {
        todo!()
    }

    fn storage_store_big_uint_raw(&self, _key: &[u8], _value_handle: Self::BigIntHandle) {
        todo!()
    }

    fn storage_store_managed_buffer_raw(
        &self,
        _key_handle: Self::ManagedBufferHandle,
        _value_handle: Self::ManagedBufferHandle,
    ) {
        todo!()
    }

    fn storage_store_managed_buffer_clear(&self, _key_handle: Self::ManagedBufferHandle) {
        todo!()
    }
}
