use crate::api::{
    const_handles, managed_types::HandleConstraints, use_raw_handle, ManagedBufferApiImpl,
    StaticVarApiImpl, StorageReadApi, StorageReadApiImpl, VMApi,
};

use super::ExternalViewApi;

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
            use_raw_handle(A::static_var_api_impl().next_handle());
        A::storage_read_api_impl().storage_load_managed_buffer_raw(
            external_view_target_key_handle,
            external_view_target_address_handle.clone(),
        );
        A::static_var_api_impl().set_external_view_target_address_handle(
            external_view_target_address_handle.get_raw_handle(),
        );
    }
}

impl<A: VMApi> StorageReadApiImpl for ExternalViewApi<A> {
    fn storage_read_api_init(&self) {
        self.load_external_view_target_key_handle();
    }

    fn storage_load_managed_buffer_raw(
        &self,
        key_handle: Self::ManagedBufferHandle,
        dest: Self::ManagedBufferHandle,
    ) {
        let target_address_handle =
            use_raw_handle(A::static_var_api_impl().get_external_view_target_address_handle());
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
