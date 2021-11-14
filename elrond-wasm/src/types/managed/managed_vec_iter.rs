use crate::api::ManagedTypeApi;

use super::{ManagedType, ManagedVec, ManagedVecItem};

impl<'a, M, T> IntoIterator for &'a ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    type Item = T;
    type IntoIter = ManagedVecIterator<'a, M, T>;
    fn into_iter(self) -> Self::IntoIter {
        ManagedVecIterator::new(self)
    }
}

pub struct ManagedVecIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    managed_vec: &'a ManagedVec<M, T>,
    byte_start: usize,
    byte_end: usize,
}

impl<'a, M, T> ManagedVecIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    pub(crate) fn new(managed_vec: &'a ManagedVec<M, T>) -> Self {
        ManagedVecIterator {
            managed_vec,
            byte_start: 0,
            byte_end: managed_vec.byte_len(),
        }
    }

    #[inline]
    pub(crate) fn type_manager(&self) -> M {
        self.managed_vec.type_manager()
    }
}

impl<'a, M, T> Iterator for ManagedVecIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let next_byte_start = self.byte_start + T::PAYLOAD_SIZE;
        if next_byte_start > self.byte_end {
            return None;
        }
        let result = T::from_byte_reader(self.type_manager(), |dest_slice| {
            let _ = self
                .managed_vec
                .buffer
                .load_slice(self.byte_start, dest_slice);
        });

        self.byte_start = next_byte_start;
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.byte_end - self.byte_start) / T::PAYLOAD_SIZE;
        (remaining, Some(remaining))
    }
}

impl<'a, M, T> ExactSizeIterator for ManagedVecIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
}

impl<'a, M, T> DoubleEndedIterator for ManagedVecIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.byte_start + T::PAYLOAD_SIZE > self.byte_end {
            return None;
        }
        self.byte_end -= T::PAYLOAD_SIZE;

        let result = T::from_byte_reader(self.type_manager(), |dest_slice| {
            let _ = self
                .managed_vec
                .buffer
                .load_slice(self.byte_end, dest_slice);
        });

        Some(result)
    }
}

impl<'a, M, T> Clone for ManagedVecIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    #[allow(clippy::clone_double_ref)]
    fn clone(&self) -> Self {
        Self {
            managed_vec: self.managed_vec,
            byte_start: self.byte_start,
            byte_end: self.byte_end,
        }
    }
}
