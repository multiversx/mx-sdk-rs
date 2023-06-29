use super::VmApiImpl;
use multiversx_sc::api::{
    StorageReadApi, StorageReadApiImpl, StorageWriteApi, StorageWriteApiImpl,
};

extern "C" {
    // managed buffer API
    fn mBufferStorageStore(keyHandle: i32, mBufferHandle: i32) -> i32;
    fn mBufferStorageLoad(keyHandle: i32, mBufferHandle: i32) -> i32;

    // from another account
    fn mBufferStorageLoadFromAddress(addressHandle: i32, keyHandle: i32, mBufferHandle: i32);
}

impl StorageReadApi for VmApiImpl {
    type StorageReadApiImpl = VmApiImpl;

    #[inline]
    fn storage_read_api_impl() -> Self::StorageReadApiImpl {
        VmApiImpl {}
    }
}

impl StorageReadApiImpl for VmApiImpl {
    #[inline]
    fn storage_load_managed_buffer_raw(
        &self,
        key_handle: Self::ManagedBufferHandle,
        dest: Self::ManagedBufferHandle,
    ) {
        unsafe {
            mBufferStorageLoad(key_handle, dest);
        }
    }

    #[inline]
    fn storage_load_from_address(
        &self,
        address_handle: Self::ManagedBufferHandle,
        key_handle: Self::ManagedBufferHandle,
        dest: Self::ManagedBufferHandle,
    ) {
        unsafe {
            mBufferStorageLoadFromAddress(address_handle, key_handle, dest);
        }
    }
}

impl StorageWriteApi for VmApiImpl {
    type StorageWriteApiImpl = VmApiImpl;

    #[inline]
    fn storage_write_api_impl() -> Self::StorageWriteApiImpl {
        VmApiImpl {}
    }
}

impl StorageWriteApiImpl for VmApiImpl {
    fn storage_store_managed_buffer_raw(
        &self,
        key_handle: Self::ManagedBufferHandle,
        value_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            mBufferStorageStore(key_handle, value_handle);
        }
    }
}
