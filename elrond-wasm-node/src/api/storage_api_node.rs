use super::VmApiImpl;
use elrond_wasm::{
    api::{Handle, StorageReadApi, StorageReadApiImpl, StorageWriteApi, StorageWriteApiImpl},
    types::heap::{Box, BoxedBytes},
};

#[rustfmt::skip]
extern "C" {
	// general
	fn storageStore(keyOffset: *const u8, keyLength: i32, dataOffset: *const u8, dataLength: i32) -> i32;
	fn storageLoadLength(keyOffset: *const u8, keyLength: i32) -> i32;
	fn storageLoad(keyOffset: *const u8, keyLength: i32, dataOffset: *mut u8) -> i32;

	// big int API
	fn bigIntNew(value: i64) -> i32;
	fn bigIntStorageStoreUnsigned(keyOffset: *const u8, keyLength: i32, source: i32) -> i32;
	fn bigIntStorageLoadUnsigned(keyOffset: *const u8, keyLength: i32, destination: i32) -> i32;

	// small int API
	fn smallIntStorageStoreUnsigned(keyOffset: *const u8, keyLength: i32, value: i64) -> i32;
	fn smallIntStorageStoreSigned(keyOffset: *const u8, keyLength: i32, value: i64) -> i32;
	fn smallIntStorageLoadUnsigned(keyOffset: *const u8, keyLength: i32) -> i64;
	fn smallIntStorageLoadSigned(keyOffset: *const u8, keyLength: i32) -> i64;

    // managed buffer API
    fn mBufferNew() -> i32;
    fn mBufferStorageStore(keyHandle: i32, mBufferHandle: i32) -> i32;
    fn mBufferStorageLoad(keyHandle: i32, mBufferHandle: i32) -> i32;
    fn mBufferGetLength(mBufferHandle: i32) -> i32;
    
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
    fn storage_load_big_uint_raw(&self, key: &[u8]) -> i32 {
        unsafe {
            let handle = bigIntNew(0);
            bigIntStorageLoadUnsigned(key.as_ref().as_ptr(), key.len() as i32, handle);
            handle
        }
    }

    #[inline]
    fn storage_load_managed_buffer_raw(&self, key_handle: Handle) -> Handle {
        unsafe {
            let value_handle = mBufferNew();
            mBufferStorageLoad(key_handle, value_handle);
            value_handle
        }
    }

    #[inline]
    fn storage_load_managed_buffer_len(&self, key_handle: Handle) -> usize {
        unsafe {
            // TODO: use a temp handle
            let value_handle = mBufferNew();
            mBufferStorageLoad(key_handle, value_handle);
            mBufferGetLength(value_handle) as usize
        }
    }

    #[inline]
    fn storage_load_u64(&self, key: &[u8]) -> u64 {
        unsafe { smallIntStorageLoadUnsigned(key.as_ref().as_ptr(), key.len() as i32) as u64 }
    }

    #[inline]
    fn storage_load_i64(&self, key: &[u8]) -> i64 {
        unsafe { smallIntStorageLoadSigned(key.as_ref().as_ptr(), key.len() as i32) }
    }

    #[inline]
    fn storage_load_from_address(&self, address_handle: Handle, key_handle: Handle) -> Handle {
        unsafe {
            let value_handle = mBufferNew();
            mBufferStorageLoadFromAddress(address_handle, key_handle, value_handle);
            value_handle
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
    fn storage_store_big_uint_raw(&self, key: &[u8], handle: i32) {
        unsafe {
            bigIntStorageStoreUnsigned(key.as_ref().as_ptr(), key.len() as i32, handle);
        }
    }

    fn storage_store_managed_buffer_raw(&self, key_handle: Handle, value_handle: Handle) {
        unsafe {
            mBufferStorageStore(key_handle, value_handle);
        }
    }

    fn storage_store_managed_buffer_clear(&self, key_handle: Handle) {
        unsafe {
            let value_handle = mBufferNew();
            mBufferStorageStore(key_handle, value_handle);
        }
    }

    #[inline]
    fn storage_store_u64(&self, key: &[u8], value: u64) {
        unsafe {
            smallIntStorageStoreUnsigned(key.as_ref().as_ptr(), key.len() as i32, value as i64);
        }
    }

    #[inline]
    fn storage_store_i64(&self, key: &[u8], value: i64) {
        unsafe {
            smallIntStorageStoreSigned(key.as_ref().as_ptr(), key.len() as i32, value);
        }
    }
}
