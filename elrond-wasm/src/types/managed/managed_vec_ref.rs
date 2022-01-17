use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::api::{Handle, ManagedTypeApi};

use super::{ManagedType, ManagedVec, ManagedVecItem};

pub struct ManagedVecRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    _phantom_m: PhantomData<M>,
    _phantom_t: PhantomData<&'a mut T>, // needed for the lifetime, even though T is present
    managed_vec_handle: Handle,
    item_index: usize,
    item: T,
}

impl<'a, M, T> ManagedVecRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    #[inline]
    fn wrap_as_managed_vec(managed_vec_handle: Handle) -> ManagedVec<M, T> {
        ManagedVec::from_raw_handle(managed_vec_handle)
    }

    pub(super) fn new(managed_vec_handle: Handle, item_index: usize) -> Self {
        let item = unsafe { Self::wrap_as_managed_vec(managed_vec_handle).get_unsafe(item_index) };
        Self {
            _phantom_m: PhantomData,
            _phantom_t: PhantomData,
            managed_vec_handle,
            item_index,
            item,
        }
    }
}

impl<'a, M, T> Drop for ManagedVecRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    fn drop(&mut self) {
        let _ = Self::wrap_as_managed_vec(self.managed_vec_handle).set(self.item_index, &self.item);
    }
}

impl<'a, M, T> Deref for ManagedVecRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<'a, M, T> DerefMut for ManagedVecRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}
