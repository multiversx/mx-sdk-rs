use core::marker::PhantomData;

use crate::{api::ManagedTypeApi, types::ManagedType};

use super::{ManagedVec, ManagedVecItem, ManagedVecPayloadIterator};

impl<'a, M, T> IntoIterator for &'a ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    type Item = T::Ref<'a>;
    type IntoIter = ManagedVecRefIterator<'a, M, T>;
    fn into_iter(self) -> Self::IntoIter {
        ManagedVecRefIterator::new(self)
    }
}

pub struct ManagedVecRefIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    payload_iter: ManagedVecPayloadIterator<M, T::PAYLOAD>,
    _phantom: PhantomData<&'a ManagedVec<M, T>>,
}

impl<'a, M, T> ManagedVecRefIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    pub(crate) fn new(managed_vec: &'a ManagedVec<M, T>) -> Self {
        unsafe {
            ManagedVecRefIterator {
                payload_iter: ManagedVecPayloadIterator::new(managed_vec.get_handle()),
                _phantom: PhantomData,
            }
        }
    }
}

impl<'a, M, T> Iterator for ManagedVecRefIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    type Item = T::Ref<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let payload = self.payload_iter.next()?;
        unsafe { Some(T::borrow_from_payload(&payload)) }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.payload_iter.size_hint()
    }
}

impl<M, T> ExactSizeIterator for ManagedVecRefIterator<'_, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
}

impl<M, T> DoubleEndedIterator for ManagedVecRefIterator<'_, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let payload = self.payload_iter.next_back()?;
        unsafe { Some(T::borrow_from_payload(&payload)) }
    }
}

impl<M, T> Clone for ManagedVecRefIterator<'_, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    fn clone(&self) -> Self {
        unsafe {
            ManagedVecRefIterator {
                payload_iter: self.payload_iter.clone_iter(),
                _phantom: PhantomData,
            }
        }
    }
}
