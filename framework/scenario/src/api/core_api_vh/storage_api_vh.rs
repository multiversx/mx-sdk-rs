use multiversx_sc::api::{
    HandleConstraints, StorageReadApi, StorageReadApiImpl, StorageWriteApi, StorageWriteApiImpl,
};

use crate::api::{VMHooksApi, VMHooksApiBackend};

impl<VHB: VMHooksApiBackend> StorageReadApi for VMHooksApi<VHB> {
    type StorageReadApiImpl = Self;

    fn storage_read_api_impl() -> Self::StorageReadApiImpl {
        Self::api_impl()
    }
}

impl<VHB: VMHooksApiBackend> StorageReadApiImpl for VMHooksApi<VHB> {
    fn storage_load_managed_buffer_raw(
        &self,
        key_handle: Self::ManagedBufferHandle,
        dest: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.mbuffer_storage_load(key_handle.get_raw_handle(), dest.get_raw_handle())
        });
    }

    fn storage_load_from_address(
        &self,
        address_handle: Self::ManagedBufferHandle,
        key_handle: Self::ManagedBufferHandle,
        dest: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.mbuffer_storage_load_from_address(
                address_handle.get_raw_handle(),
                key_handle.get_raw_handle(),
                dest.get_raw_handle(),
            );
        })
    }
}

impl<VHB: VMHooksApiBackend> StorageWriteApi for VMHooksApi<VHB> {
    type StorageWriteApiImpl = Self;

    fn storage_write_api_impl() -> Self::StorageWriteApiImpl {
        Self::api_impl()
    }
}

impl<VHB: VMHooksApiBackend> StorageWriteApiImpl for VMHooksApi<VHB> {
    fn storage_store_managed_buffer_raw(
        &self,
        key_handle: Self::ManagedBufferHandle,
        value_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.mbuffer_storage_store(key_handle.get_raw_handle(), value_handle.get_raw_handle());
        });
    }
}
