use multiversx_sc::api::ManagedMapApiImpl;

use crate::api::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> ManagedMapApiImpl for VMHooksApi<BACKEND_TYPE> {
    fn mm_new(&self) -> Self::ManagedBufferHandle {
        todo!()
    }

    fn mm_get(
        &self,
        _map_handle: Self::ManagedMapHandle,
        _key_handle: Self::ManagedBufferHandle,
        _value_handle: Self::ManagedBufferHandle,
    ) {
        todo!()
    }

    fn mm_put(
        &self,
        _map_handle: Self::ManagedMapHandle,
        _key_handle: Self::ManagedBufferHandle,
        _out_value_handle: Self::ManagedBufferHandle,
    ) {
        todo!()
    }

    fn mm_remove(
        &self,
        _map_handle: Self::ManagedMapHandle,
        _key_handle: Self::ManagedBufferHandle,
        _out_value_handle: Self::ManagedBufferHandle,
    ) {
        todo!()
    }

    fn mm_contains(
        &self,
        _map_handle: Self::ManagedMapHandle,
        _key_handle: Self::ManagedBufferHandle,
    ) -> bool {
        todo!()
    }
}
