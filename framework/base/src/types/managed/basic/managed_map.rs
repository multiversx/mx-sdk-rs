use crate::{
    api::{ManagedMapApiImpl, ManagedTypeApi, StaticVarApiImpl, use_raw_handle},
    types::ManagedType,
};
use crate::api::HandleConstraints;

use super::ManagedBuffer;

/// A byte buffer managed by an external API.
#[repr(transparent)]
pub struct ManagedMap<M: ManagedTypeApi> {
    pub(crate) handle: M::ManagedMapHandle,
}

impl<M: ManagedTypeApi> ManagedType<M> for ManagedMap<M> {
    type OwnHandle = M::ManagedMapHandle;

    #[inline]
    fn from_handle(handle: M::ManagedMapHandle) -> Self {
        ManagedMap { handle }
    }

    fn get_handle(&self) -> &M::ManagedMapHandle {
        &self.handle
    }

    fn take_handle(self) -> Self::OwnHandle {
        self.handle.take_handle()
    }

    fn take_handle_ref(&mut self) -> Self::OwnHandle {
        self.handle.take_handle_ref()
    }

    fn transmute_from_handle_ref(handle_ref: &M::ManagedMapHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> ManagedMap<M> {
    pub fn new() -> Self {
        let new_handle = M::managed_type_impl().mm_new();
        ManagedMap::from_handle(new_handle)
    }
}

impl<M: ManagedTypeApi> Default for ManagedMap<M> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<M: ManagedTypeApi> ManagedMap<M> {
    pub fn get(&self, key: &ManagedBuffer<M>) -> ManagedBuffer<M> {
        let new_handle: M::ManagedBufferHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().mm_get(&self.handle, &key.handle, &new_handle);
        ManagedBuffer::from_handle(new_handle)
    }

    pub fn put(&mut self, key: &ManagedBuffer<M>, value: &ManagedBuffer<M>) {
        M::managed_type_impl().mm_put(
            &self.handle,
            &key.handle,
            &value.handle,
        );
    }

    pub fn remove(&mut self, key: &ManagedBuffer<M>) -> ManagedBuffer<M> {
        let new_handle: M::ManagedBufferHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().mm_remove(
            &self.handle,
            &key.handle,
            &new_handle,
        );
        ManagedBuffer::from_handle(new_handle)
    }

    pub fn contains(&self, key: &ManagedBuffer<M>) -> bool {
        M::managed_type_impl().mm_contains(&self.handle, &key.handle)
    }
}
