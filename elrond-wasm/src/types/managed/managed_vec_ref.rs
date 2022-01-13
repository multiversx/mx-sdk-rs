use core::ops::{Deref, DerefMut};

use crate::api::{ErrorApiImpl, ManagedTypeApi};

use super::{managed_vec::INDEX_OUT_OF_RANGE_MSG, ManagedVec, ManagedVecItem};

pub struct ManagedVecRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    managed_vec: &'a mut ManagedVec<M, T>,
    item_index: usize,
    item: T,
}

impl<'a, M, T> ManagedVecRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    pub(crate) fn new(managed_vec: &'a mut ManagedVec<M, T>, item_index: usize) -> Self {
        let item = match managed_vec.try_get(item_index) {
            Some(t) => t,
            None => M::error_api_impl().signal_error(INDEX_OUT_OF_RANGE_MSG),
        };

        Self {
            managed_vec,
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
        let _ = self.managed_vec.set(self.item_index, &self.item);
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
