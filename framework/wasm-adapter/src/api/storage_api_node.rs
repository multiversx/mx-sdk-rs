use super::VmApiImpl;
use multiversx_sc::{
    api::{
        const_handles, StorageReadApi, StorageReadApiImpl, StorageWriteApi, StorageWriteApiImpl,
    },
    types::heap::{Box, BoxedBytes},
};

#[rustfmt::skip]
extern "C" {
	// general
	fn storageStore(keyOffset: *const u8, keyLength: i32, dataOffset: *const u8, dataLength: i32) -> i32;
	fn storageLoadLength(keyOffset: *const u8, keyLength: i32) -> i32;
	fn storageLoad(keyOffset: *const u8, keyLength: i32, dataOffset: *mut u8) -> i32;

	// big int API
	fn bigIntStorageStoreUnsigned(keyOffset: *const u8, keyLength: i32, source: i32) -> i32;
	fn bigIntStorageLoadUnsigned(keyOffset: *const u8, keyLength: i32, destination: i32) -> i32;

    // managed buffer API
    fn mBufferSetBytes(mBufferHandle: i32, byte_ptr: *const u8, byte_len: i32) -> i32;
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
    fn storage_load_len(&self, key: &[u8]) -> usize {
        unsafe { storageLoadLength(key.as_ref().as_ptr(), key.len() as i32) as usize }
    }

    fn storage_load_to_heap(&self, key: &[u8]) -> Box<[u8]> {
        let len = self.storage_load_len(key);
        unsafe {
            let mut res = BoxedBytes::allocate(len);
            if len > 0 {
                storageLoad(key.as_ref().as_ptr(), key.len() as i32, res.as_mut_ptr());
            }
            res.into_box()
        }
    }

    #[inline]
    fn storage_load_big_uint_raw(&self, key: &[u8], dest: Self::ManagedBufferHandle) {
        unsafe {
            bigIntStorageLoadUnsigned(key.as_ref().as_ptr(), key.len() as i32, dest);
        }
    }

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
    fn storage_store_big_uint_raw(&self, key: &[u8], value_handle: Self::BigIntHandle) {
        unsafe {
            bigIntStorageStoreUnsigned(key.as_ref().as_ptr(), key.len() as i32, value_handle);
        }
    }

    fn storage_store_managed_buffer_raw(
        &self,
        key_handle: Self::ManagedBufferHandle,
        value_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            mBufferStorageStore(key_handle, value_handle);
        }
    }

    fn storage_store_managed_buffer_clear(&self, key_handle: Self::ManagedBufferHandle) {
        unsafe {
            // TODO: this will no longer be necessay once the ("no managed buffer under the given handle" is removed from VM
            let _ = mBufferSetBytes(const_handles::MBUF_CONST_EMPTY, core::ptr::null(), 0);
            mBufferStorageStore(key_handle, const_handles::MBUF_CONST_EMPTY);
        }
    }
}
