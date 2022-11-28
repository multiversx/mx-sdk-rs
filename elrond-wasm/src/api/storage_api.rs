use alloc::boxed::Box;

use super::HandleTypeInfo;

pub trait StorageReadApi: HandleTypeInfo {
    type StorageReadApiImpl: StorageReadApiImpl
        + HandleTypeInfo<
            ManagedBufferHandle = Self::ManagedBufferHandle,
            BigIntHandle = Self::BigIntHandle,
            BigFloatHandle = Self::BigFloatHandle,
            EllipticCurveHandle = Self::EllipticCurveHandle,
        >;

    fn storage_read_api_impl() -> Self::StorageReadApiImpl;
}

pub trait StorageReadApiImpl: HandleTypeInfo {
    fn storage_read_api_init(&self) {}

    fn storage_load_len(&self, key: &[u8]) -> usize;

    fn storage_load_to_heap(&self, key: &[u8]) -> Box<[u8]>;

    fn storage_load_big_uint_raw(&self, key: &[u8], dest: Self::BigIntHandle);

    fn storage_load_managed_buffer_raw(
        &self,
        key_handle: Self::ManagedBufferHandle,
        dest: Self::ManagedBufferHandle,
    );

    fn storage_load_from_address(
        &self,
        address_handle: Self::ManagedBufferHandle,
        key_handle: Self::ManagedBufferHandle,
        dest: Self::ManagedBufferHandle,
    );
}

pub trait StorageWriteApi: HandleTypeInfo {
    type StorageWriteApiImpl: StorageWriteApiImpl
        + HandleTypeInfo<
            ManagedBufferHandle = Self::ManagedBufferHandle,
            BigIntHandle = Self::BigIntHandle,
            BigFloatHandle = Self::BigFloatHandle,
            EllipticCurveHandle = Self::EllipticCurveHandle,
        >;

    fn storage_write_api_impl() -> Self::StorageWriteApiImpl;
}

pub trait StorageWriteApiImpl: HandleTypeInfo {
    fn storage_store_slice_u8(&self, key: &[u8], value: &[u8]);

    fn storage_store_big_uint_raw(&self, key: &[u8], value_handle: Self::BigIntHandle);

    fn storage_store_managed_buffer_raw(
        &self,
        key_handle: Self::ManagedBufferHandle,
        value_handle: Self::ManagedBufferHandle,
    );

    fn storage_store_managed_buffer_clear(&self, key_handle: Self::ManagedBufferHandle);
}
