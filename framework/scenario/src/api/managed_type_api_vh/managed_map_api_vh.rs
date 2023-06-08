use multiversx_sc::api::ManagedMapApi;

use crate::api::VMHooksApiImpl;

impl ManagedMapApi for VMHooksApiImpl {
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
