use crate::api::ManagedTypeApi;

use super::{ManagedVec, ManagedVecItem, ManagedVecItemPayload};

impl<'a, M, T> IntoIterator for &'a ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    type Item = T;
    type IntoIter = ManagedVecOwnedIterator<'a, M, T>;
    fn into_iter(self) -> Self::IntoIter {
        ManagedVecOwnedIterator::new(self)
    }
}

pub struct ManagedVecOwnedIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    managed_vec: &'a ManagedVec<M, T>,
    byte_start: usize,
    byte_end: usize,
}

impl<'a, M, T> ManagedVecOwnedIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    pub(crate) fn new(managed_vec: &'a ManagedVec<M, T>) -> Self {
        let byte_end = managed_vec.byte_len();
        ManagedVecOwnedIterator {
            managed_vec,
            byte_start: 0,
            byte_end,
        }
    }
}

impl<M, T> Iterator for ManagedVecOwnedIterator<'_, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // managedrev<t>  / reference type
        let next_byte_start = self.byte_start + T::payload_size();
        if next_byte_start > self.byte_end {
            return None;
        }

        let mut payload = T::PAYLOAD::new_buffer();
        let _ = self
            .managed_vec
            .buffer
            .load_slice(self.byte_start, payload.payload_slice_mut());

        self.byte_start = next_byte_start;
        Some(T::read_from_payload(&payload))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = T::payload_size();
        let remaining = (self.byte_end - self.byte_start) / size;
        (remaining, Some(remaining))
    }
}

impl<M, T> ExactSizeIterator for ManagedVecOwnedIterator<'_, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
}

impl<M, T> DoubleEndedIterator for ManagedVecOwnedIterator<'_, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.byte_start + T::payload_size() > self.byte_end {
            return None;
        }
        self.byte_end -= T::payload_size();

        let mut payload = T::PAYLOAD::new_buffer();
        let _ = self
            .managed_vec
            .buffer
            .load_slice(self.byte_end, payload.payload_slice_mut());

        Some(T::read_from_payload(&payload))
    }
}

impl<M, T> Clone for ManagedVecOwnedIterator<'_, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    fn clone(&self) -> Self {
        let byte_end = self.byte_end;
        Self {
            managed_vec: self.managed_vec,
            byte_start: self.byte_start,
            byte_end,
        }
    }
}
