use crate::{api::ManagedTypeApi, types::ManagedType};

use super::{ManagedVec, ManagedVecItem, ManagedVecPayloadIterator};

impl<M, T> IntoIterator for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    type Item = T;
    type IntoIter = ManagedVecOwnedIterator<M, T>;
    fn into_iter(self) -> Self::IntoIter {
        ManagedVecOwnedIterator::new(self)
    }
}

pub struct ManagedVecOwnedIterator<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    payload_iter: ManagedVecPayloadIterator<M, T::PAYLOAD>,
}

impl<M, T> ManagedVecOwnedIterator<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    pub(crate) fn new(managed_vec: ManagedVec<M, T>) -> Self {
        unsafe {
            ManagedVecOwnedIterator {
                payload_iter: ManagedVecPayloadIterator::new(managed_vec.forget_into_handle()),
            }
        }
    }
}

impl<M, T> Iterator for ManagedVecOwnedIterator<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let payload = self.payload_iter.next()?;
        Some(T::read_from_payload(&payload))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.payload_iter.size_hint()
    }
}

impl<M, T> ExactSizeIterator for ManagedVecOwnedIterator<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
}

impl<M, T> DoubleEndedIterator for ManagedVecOwnedIterator<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let payload = self.payload_iter.next_back()?;
        Some(T::read_from_payload(&payload))
    }
}
