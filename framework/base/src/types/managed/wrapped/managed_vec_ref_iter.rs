use crate::api::ManagedTypeApi;

use super::{ManagedVec, ManagedVecItem};

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
        let next_byte_start = self.byte_start + T::PAYLOAD_SIZE;
        if next_byte_start > self.byte_end {
            return None;
        }
        let result = unsafe {
            T::from_byte_reader_as_borrow(|dest_slice| {
                let _ = self
                    .managed_vec
                    .buffer
                    .load_slice(self.byte_start, dest_slice);
            })
        };

        self.byte_start = next_byte_start;
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.byte_end - self.byte_start) / T::PAYLOAD_SIZE;
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
        if self.byte_start + T::PAYLOAD_SIZE > self.byte_end {
            return None;
        }
        self.byte_end -= T::PAYLOAD_SIZE;

        let result = unsafe {
            T::from_byte_reader_as_borrow(|dest_slice| {
                let _ = self
                    .managed_vec
                    .buffer
                    .load_slice(self.byte_end, dest_slice);
            })
        };

        Some(result)
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
