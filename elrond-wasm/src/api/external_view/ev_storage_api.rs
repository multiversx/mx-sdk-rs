use crate::api::{
    const_handles, use_raw_handle, ManagedBufferApi, ManagedTypeApiImpl, StaticVarApiImpl,
    StorageReadApi, StorageReadApiImpl, VMApi,
};

use super::ExternalViewApi;
use alloc::boxed::Box;

pub const EXTERNAL_VIEW_TARGET_ADRESS_KEY: &[u8] = b"external-view-target-address";

impl<A> StorageReadApi for ExternalViewApi<A>
where
    A: VMApi,
{
    type StorageReadApiImpl = ExternalViewApi<A>;

    fn storage_read_api_impl() -> Self::StorageReadApiImpl {
        ExternalViewApi::new()
    }
}

impl<A> ExternalViewApi<A>
where
    A: VMApi,
{
    /// Reads what lies in storage at `external-view-target-address` and loads into a managed buffer.
    /// The same managed buffer will be used for all reads in the tx.
    fn load_external_view_target_key_handle(&self) {
        let external_view_target_key_handle: A::ManagedBufferHandle =
            use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        A::managed_type_impl().mb_overwrite(
            external_view_target_key_handle.clone(),
            EXTERNAL_VIEW_TARGET_ADRESS_KEY,
        );
        let external_view_target_address_handle: A::ManagedBufferHandle =
            A::static_var_api_impl().next_handle();
        A::storage_read_api_impl().storage_load_managed_buffer_raw(
            external_view_target_key_handle,
            external_view_target_address_handle.clone(),
        );
        A::static_var_api_impl()
            .set_external_view_target_address_handle(external_view_target_address_handle);
    }
}

impl<A: VMApi> StorageReadApiImpl for ExternalViewApi<A> {
    fn storage_read_api_init(&self) {
        self.load_external_view_target_key_handle();
    }

    fn storage_load_len(&self, key: &[u8]) -> usize {
        let mbuf_temp_1: A::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        A::managed_type_impl().mb_overwrite(mbuf_temp_1.clone(), key);
        let mbuf_temp_2: A::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_2);
        self.storage_load_managed_buffer_raw(mbuf_temp_1, mbuf_temp_2.clone());
        A::managed_type_impl().mb_len(mbuf_temp_2)
    }

    fn storage_load_to_heap(&self, key: &[u8]) -> Box<[u8]> {
        let mbuf_temp_1: A::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        A::managed_type_impl().mb_overwrite(mbuf_temp_1.clone(), key);
        let mbuf_temp_2: A::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_2);
        self.storage_load_managed_buffer_raw(mbuf_temp_1, mbuf_temp_2.clone());
        A::managed_type_impl()
            .mb_to_boxed_bytes(mbuf_temp_2)
            .into_box()
    }

    fn storage_load_big_uint_raw(&self, key: &[u8], dest: Self::BigIntHandle) {
        let mbuf_temp_1: A::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        A::managed_type_impl().mb_overwrite(mbuf_temp_1.clone(), key);
        let mbuf_temp_2: A::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_2);
        self.storage_load_managed_buffer_raw(mbuf_temp_1, mbuf_temp_2.clone());
        A::managed_type_impl().mb_to_big_int_unsigned(mbuf_temp_2, dest)
    }

    fn storage_load_managed_buffer_raw(
        &self,
        key_handle: Self::ManagedBufferHandle,
        dest: Self::ManagedBufferHandle,
    ) {
        let target_address_handle =
            A::static_var_api_impl().get_external_view_target_address_handle();
        A::storage_read_api_impl().storage_load_from_address(
            target_address_handle,
            key_handle,
            dest,
        );
    }

    fn storage_load_from_address(
        &self,
        address_handle: Self::ManagedBufferHandle,
        key_handle: Self::ManagedBufferHandle,
        dest: Self::ManagedBufferHandle,
    ) {
        A::storage_read_api_impl().storage_load_from_address(address_handle, key_handle, dest);
    }
}
