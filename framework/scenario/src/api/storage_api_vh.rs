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
        panic!("storage_load_len currently not implemented")
    }

    fn storage_load_to_heap(&self, _key: &[u8]) -> Box<[u8]> {
        panic!("storage_load_to_heap currently not implemented")
    }

    fn storage_load_managed_buffer_raw(
        &self,
        key_handle: Self::ManagedBufferHandle,
        dest: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks(|vh| vh.mbuffer_storage_load(key_handle, dest));
    }

    fn storage_load_from_address(
        &self,
        address_handle: Self::ManagedBufferHandle,
        key_handle: Self::ManagedBufferHandle,
        dest: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.mbuffer_storage_load_from_address(address_handle, key_handle, dest);
        })
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
        panic!("storage_store_slice_u8 currently not implemented")
    }

    fn storage_store_managed_buffer_raw(
        &self,
        key_handle: Self::ManagedBufferHandle,
        value_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.mbuffer_storage_store(key_handle, value_handle);
        });
    }
}
