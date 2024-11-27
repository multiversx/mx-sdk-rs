use crate::{
    api::ManagedTypeApi,
    types::{ManagedVec, ManagedVecItem},
};
use core::{
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
};

use super::{ManagedRef, ManagedRefMut};

pub struct ManagedVecRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    _phantom_m: PhantomData<M>,
    _phantom_t: PhantomData<&'a mut T>, // needed for the lifetime, even though T is present
    managed_vec_handle: M::ManagedBufferHandle,
    item_index: usize,
    item: ManuallyDrop<T>,
}

impl<'a, M, T> ManagedVecRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    #[inline]
    unsafe fn wrap_as_managed_vec(
        managed_vec_handle: M::ManagedBufferHandle,
    ) -> ManagedRef<'static, M, ManagedVec<M, T>> {
        ManagedRef::wrap_handle(managed_vec_handle)
    }

    pub(super) fn new(managed_vec_handle: M::ManagedBufferHandle, item_index: usize) -> Self {
        let item =
            unsafe { Self::wrap_as_managed_vec(managed_vec_handle.clone()).get_unsafe(item_index) };
        Self {
            _phantom_m: PhantomData,
            _phantom_t: PhantomData,
            managed_vec_handle,
            item_index,
            item: ManuallyDrop::new(item),
        }
    }
}

impl<'a, M, T> Drop for ManagedVecRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    fn drop(&mut self) {
        let item = unsafe { ManuallyDrop::take(&mut self.item) };
        unsafe {
            let _ =
                ManagedRefMut::<M, ManagedVec<M, T>>::wrap_handle(self.managed_vec_handle.clone())
                    .set(self.item_index, item);
        }
        // core::mem::forget(item);
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
