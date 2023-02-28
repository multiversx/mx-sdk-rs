use crate::DebugApi;
use multiversx_sc::{
    api::{HandleTypeInfo, InvalidSliceError, ManagedBufferApi, ManagedMapApi},
    types::heap::BoxedBytes,
};

impl ManagedMapApi for DebugApi {
    fn mm_new(&self) -> Self::ManagedBufferHandle {
        todo!()
    }

    fn mm_get(
        &self,
        key_handle: Self::ManagedBufferHandle,
        value_handle: Self::ManagedBufferHandle,
    ) {
        todo!()
    }

    fn mm_put(
        &self,
        key_handle: Self::ManagedBufferHandle,
        out_value_handle: Self::ManagedBufferHandle,
    ) {
        todo!()
    }

    fn mm_remove(
        &self,
        key_handle: Self::ManagedBufferHandle,
        out_value_handle: Self::ManagedBufferHandle,
    ) {
        todo!()
    }

    fn mm_contains(&self, key_handle: Self::ManagedBufferHandle) -> bool {
        todo!()
    }
}
