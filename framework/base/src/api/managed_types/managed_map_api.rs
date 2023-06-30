use super::HandleTypeInfo;

/// A raw bytes buffer managed by Arwen.
pub trait ManagedMapApiImpl: HandleTypeInfo {
    /// Requests a new handle from the VM.
    fn mm_new(&self) -> Self::ManagedMapHandle;

    fn mm_get(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
        value_handle: Self::ManagedBufferHandle,
    );

    fn mm_put(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
        out_value_handle: Self::ManagedBufferHandle,
    );

    fn mm_remove(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
        out_value_handle: Self::ManagedBufferHandle,
    );

    fn mm_contains(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
    ) -> bool;
}
