use crate::api::{
    ErrorApiImpl, Handle, ManagedBufferApi, ManagedTypeApiImpl, StaticVarApiImpl, StorageReadApi,
    StorageReadApiImpl, VMApi,
};

use super::ExternalViewApi;
use alloc::vec::Vec;

pub const EXTERNAL_VIEW_TARGET_ADRESS_KEY: &[u8] = b"external-view-target-address";

impl<A: VMApi> StorageReadApi for ExternalViewApi<A> {
    type StorageReadApiImpl = ExternalViewApi<A>;

    fn storage_read_api_impl() -> Self::StorageReadApiImpl {
        ExternalViewApi::new()
    }
}

impl<A: VMApi> StorageReadApiImpl for ExternalViewApi<A> {
    /// Reads what lies in storage at `external-view-target-address` and loads into a managed buffer.
    /// The same managed buffer will be used for all reads in the tx.
    fn storage_read_api_init(&self) {
        let external_view_target_key_handle =
            A::managed_type_impl().mb_new_from_bytes(EXTERNAL_VIEW_TARGET_ADRESS_KEY);
        let external_view_target_address_handle = A::storage_read_api_impl()
            .storage_load_managed_buffer_raw(external_view_target_key_handle);
        A::static_var_api_impl()
            .set_external_view_target_address_handle(external_view_target_address_handle);
    }

    fn storage_load_len(&self, key: &[u8]) -> usize {
        let key_handle = A::managed_type_impl().mb_new_from_bytes(key);
        self.storage_load_managed_buffer_len(key_handle)
    }

    fn storage_load_vec_u8(&self, key: &[u8]) -> Vec<u8> {
        let key_handle = A::managed_type_impl().mb_new_from_bytes(key);
        let value_handle = self.storage_load_managed_buffer_raw(key_handle);
        A::managed_type_impl()
            .mb_to_boxed_bytes(value_handle)
            .into_vec()
    }

    fn storage_load_big_uint_raw(&self, key: &[u8]) -> Handle {
        let key_handle = A::managed_type_impl().mb_new_from_bytes(key);
        let value_handle = self.storage_load_managed_buffer_raw(key_handle);
        A::managed_type_impl().mb_to_big_int_unsigned(value_handle)
    }

    fn storage_load_managed_buffer_raw(&self, key_handle: Handle) -> Handle {
        let target_address_handle =
            A::static_var_api_impl().get_external_view_target_address_handle();
        A::storage_read_api_impl().storage_load_from_address(target_address_handle, key_handle)
    }

    fn storage_load_managed_buffer_len(&self, key_handle: Handle) -> usize {
        let value_handle = self.storage_load_managed_buffer_raw(key_handle);
        A::managed_type_impl().mb_len(value_handle)
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

    fn storage_load_from_address(&self, address_handle: Handle, key_handle: Handle) -> Handle {
        A::storage_read_api_impl().storage_load_from_address(address_handle, key_handle)
    }
}
