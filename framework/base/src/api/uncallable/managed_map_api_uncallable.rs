use crate::api::ManagedMapApiImpl;

impl ManagedMapApiImpl for super::UncallableApi {
    fn mm_new(&self) -> Self::ManagedBufferHandle {
        unreachable!()
    }

    fn mm_get(
        &self,
        _map_handle: Self::ManagedMapHandle,
        _key_handle: Self::ManagedBufferHandle,
        _value_handle: Self::ManagedBufferHandle,
    ) {
        unreachable!()
    }

    fn mm_put(
        &self,
        _map_handle: Self::ManagedMapHandle,
        _key_handle: Self::ManagedBufferHandle,
        _out_value_handle: Self::ManagedBufferHandle,
    ) {
        unreachable!()
    }

    fn mm_remove(
        &self,
        _map_handle: Self::ManagedMapHandle,
        _key_handle: Self::ManagedBufferHandle,
        _out_value_handle: Self::ManagedBufferHandle,
    ) {
        unreachable!()
    }

    fn mm_contains(
        &self,
        _map_handle: Self::ManagedMapHandle,
        _key_handle: Self::ManagedBufferHandle,
    ) -> bool {
        unreachable!()
    }
}
