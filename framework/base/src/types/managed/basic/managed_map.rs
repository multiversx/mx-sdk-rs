use crate::{
    api::{use_raw_handle, ManagedMapApiImpl, ManagedTypeApi, StaticVarApiImpl},
    types::ManagedType,
};

use super::ManagedBuffer;

/// A byte buffer managed by an external API.
#[repr(transparent)]
pub struct ManagedMap<'a, M: ManagedTypeApi<'a>> {
    pub(crate) handle: M::ManagedMapHandle,
}

impl<'a, M: ManagedTypeApi<'a>> ManagedType<'a, M> for ManagedMap<'a, M> {
    type OwnHandle = M::ManagedMapHandle;

    #[inline]
    fn from_handle(handle: M::ManagedMapHandle) -> Self {
        ManagedMap { handle }
    }

    unsafe fn get_handle(&self) -> M::ManagedMapHandle {
        self.handle.clone()
    }

    fn take_handle(mut self) -> Self::OwnHandle {
        core::mem::take(&mut self.handle)
    }

    fn transmute_from_handle_ref(handle_ref: &M::ManagedMapHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<'a, M: ManagedTypeApi<'a>> ManagedMap<'a, M> {
    pub fn new() -> Self {
        let new_handle = M::managed_type_impl().mm_new();
        ManagedMap::from_handle(new_handle)
    }
}

impl<'a, M: ManagedTypeApi<'a>> Default for ManagedMap<'a, M> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, M: ManagedTypeApi<'a>> ManagedMap<'a, M> {
    pub fn get(&self, key: &ManagedBuffer<'a, M>) -> ManagedBuffer<'a, M> {
        let new_handle: M::ManagedBufferHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().mm_get(self.handle.clone(), key.handle.clone(), new_handle.clone());
        ManagedBuffer::from_handle(new_handle)
    }

    pub fn put(&mut self, key: &ManagedBuffer<'a, M>, value: &ManagedBuffer<'a, M>) {
        M::managed_type_impl().mm_put(
            self.handle.clone(),
            key.handle.clone(),
            value.handle.clone(),
        );
    }

    pub fn remove(&mut self, key: &ManagedBuffer<'a, M>) -> ManagedBuffer<'a, M> {
        let new_handle: M::ManagedBufferHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().mm_remove(
            self.handle.clone(),
            key.handle.clone(),
            new_handle.clone(),
        );
        ManagedBuffer::from_handle(new_handle)
    }

    pub fn contains(&self, key: &ManagedBuffer<'a, M>) -> bool {
        M::managed_type_impl().mm_contains(self.handle.clone(), key.handle.clone())
    }
}
