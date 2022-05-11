use alloc::boxed::Box;

use super::Handle;

pub trait StorageReadApi {
    type StorageReadApiImpl: StorageReadApiImpl;

    fn storage_read_api_impl() -> Self::StorageReadApiImpl;
}

pub trait StorageReadApiImpl {
    fn storage_read_api_init(&self) {}

    fn storage_load_len(&self, key: &[u8]) -> usize;

    fn storage_load_to_heap(&self, key: &[u8]) -> Box<[u8]>;

    fn storage_load_big_uint_raw(&self, key: &[u8], dest: Handle);

    fn storage_load_managed_buffer_raw(&self, key_handle: Handle, dest: Handle);

    fn storage_load_u64(&self, key: &[u8]) -> u64;

    fn storage_load_i64(&self, key: &[u8]) -> i64;

    fn storage_load_from_address(&self, address_handle: Handle, key_handle: Handle, dest: Handle);
}

pub trait StorageWriteApi {
    type StorageWriteApiImpl: StorageWriteApiImpl;

    fn storage_write_api_impl() -> Self::StorageWriteApiImpl;
}

pub trait StorageWriteApiImpl {
    fn storage_store_slice_u8(&self, key: &[u8], value: &[u8]);

    fn storage_store_big_uint_raw(&self, key: &[u8], value_handle: Handle);

    fn storage_store_managed_buffer_raw(&self, key_handle: Handle, value_handle: Handle);

    fn storage_store_managed_buffer_clear(&self, key_handle: Handle);

    fn storage_store_u64(&self, key: &[u8], value: u64);

    fn storage_store_i64(&self, key: &[u8], value: i64);
}
