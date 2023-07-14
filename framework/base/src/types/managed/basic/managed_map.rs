use crate::{
    api::{use_raw_handle, ManagedMapApiImpl, ManagedTypeApi, StaticVarApiImpl},
    types::ManagedType,
};

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

    fn get_handle(&self) -> M::ManagedMapHandle {
        self.handle.clone()
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
        M::managed_type_impl().mm_get(self.handle.clone(), key.handle.clone(), new_handle.clone());
        ManagedBuffer::from_handle(new_handle)
    }

    pub fn put(&mut self, key: &ManagedBuffer<M>, value: &ManagedBuffer<M>) {
        M::managed_type_impl().mm_put(
            self.handle.clone(),
            key.handle.clone(),
            value.handle.clone(),
        );
    }

    pub fn remove(&mut self, key: &ManagedBuffer<M>) -> ManagedBuffer<M> {
        let new_handle: M::ManagedBufferHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().mm_remove(
            self.handle.clone(),
            key.handle.clone(),
            new_handle.clone(),
        );
        ManagedBuffer::from_handle(new_handle)
    }

    pub fn contains(&self, key: &ManagedBuffer<M>) -> bool {
        M::managed_type_impl().mm_contains(self.handle.clone(), key.handle.clone())
    }
}
