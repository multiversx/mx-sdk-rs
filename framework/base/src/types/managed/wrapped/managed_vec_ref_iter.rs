use crate::api::ManagedTypeApi;

use super::{ManagedVec, ManagedVecItem, ManagedVecItemPayload};

pub struct ManagedVecRefIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    managed_vec: &'a ManagedVec<M, T>,
    byte_start: usize,
    byte_end: usize,
}

impl<'a, M, T> ManagedVecRefIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    pub(crate) fn new(managed_vec: &'a ManagedVec<M, T>) -> Self {
        ManagedVecRefIterator {
            managed_vec,
            byte_start: 0,
            byte_end: managed_vec.byte_len(),
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

        unsafe { Some(T::borrow_from_payload(&payload)) }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.byte_end - self.byte_start) / T::payload_size();
        (remaining, Some(remaining))
    }
}

impl<'a, M, T> ExactSizeIterator for ManagedVecRefIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
}

impl<'a, M, T> DoubleEndedIterator for ManagedVecRefIterator<'a, M, T>
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

        unsafe { Some(T::borrow_from_payload(&payload)) }
    }
}

impl<'a, M, T> Clone for ManagedVecRefIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    fn clone(&self) -> Self {
        Self {
            managed_vec: self.managed_vec,
            byte_start: self.byte_start,
            byte_end: self.byte_end,
        }
    }
}
