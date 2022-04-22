use crate::api::{
    const_handles, ErrorApiImpl, Handle, ManagedBufferApi, ManagedTypeApiImpl, StaticVarApiImpl,
    StorageReadApi, StorageReadApiImpl, VMApi,
};

use super::ExternalViewApi;
use alloc::boxed::Box;

pub const EXTERNAL_VIEW_TARGET_ADRESS_KEY: &[u8] = b"external-view-target-address";

impl<A: VMApi> StorageReadApi for ExternalViewApi<A> {
    type StorageReadApiImpl = ExternalViewApi<A>;

    fn storage_read_api_impl() -> Self::StorageReadApiImpl {
        ExternalViewApi::new()
    }
}

impl<A: VMApi> ExternalViewApi<A> {
    /// Reads what lies in storage at `external-view-target-address` and loads into a managed buffer.
    /// The same managed buffer will be used for all reads in the tx.
    fn load_external_view_target_key_handle(&self) {
        let external_view_target_key_handle = const_handles::MBUF_TEMPORARY_1;
        A::managed_type_impl().mb_overwrite(
            external_view_target_key_handle,
            EXTERNAL_VIEW_TARGET_ADRESS_KEY,
        );
        let external_view_target_address_handle = A::static_var_api_impl().next_handle();
        A::storage_read_api_impl().storage_load_managed_buffer_raw(
            external_view_target_key_handle,
            external_view_target_address_handle,
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
        A::managed_type_impl().mb_overwrite(const_handles::MBUF_TEMPORARY_1, key);
        self.storage_load_managed_buffer_raw(
            const_handles::MBUF_TEMPORARY_1,
            const_handles::MBUF_TEMPORARY_2,
        );
        A::managed_type_impl().mb_len(const_handles::MBUF_TEMPORARY_2)
    }

    fn storage_load_to_heap(&self, key: &[u8]) -> Box<[u8]> {
        A::managed_type_impl().mb_overwrite(const_handles::MBUF_TEMPORARY_1, key);
        self.storage_load_managed_buffer_raw(
            const_handles::MBUF_TEMPORARY_1,
            const_handles::MBUF_TEMPORARY_2,
        );
        A::managed_type_impl()
            .mb_to_boxed_bytes(const_handles::MBUF_TEMPORARY_2)
            .into_box()
    }

    fn storage_load_big_uint_raw(&self, key: &[u8], dest: Handle) {
        A::managed_type_impl().mb_overwrite(const_handles::MBUF_TEMPORARY_1, key);
        self.storage_load_managed_buffer_raw(
            const_handles::MBUF_TEMPORARY_1,
            const_handles::MBUF_TEMPORARY_2,
        );
        A::managed_type_impl().mb_to_big_int_unsigned(const_handles::MBUF_TEMPORARY_2, dest)
    }

    fn storage_load_managed_buffer_raw(&self, key_handle: Handle, dest: Handle) {
        let target_address_handle =
            A::static_var_api_impl().get_external_view_target_address_handle();
        A::storage_read_api_impl().storage_load_from_address(
            target_address_handle,
            key_handle,
            dest,
        );
    }

    fn storage_load_u64(&self, _key: &[u8]) -> u64 {
        // TODO: probably unreachable, investigate whether or not we can remove forever
        A::error_api_impl()
            .signal_error(b"storage_load_u64 not implemented for external view contracts")
    }

    fn storage_load_i64(&self, _key: &[u8]) -> i64 {
        // TODO: probably unreachable, investigate whether or not we can remove forever
        A::error_api_impl()
            .signal_error(b"storage_load_i64 not implemented for external view contracts")
    }

    fn storage_load_from_address(&self, address_handle: Handle, key_handle: Handle, dest: Handle) {
        A::storage_read_api_impl().storage_load_from_address(address_handle, key_handle, dest);
    }
}
