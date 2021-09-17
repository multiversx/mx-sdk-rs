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
    byte_index: usize,
    byte_limit: usize,
}

impl<'a, M, T> ManagedVecIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    pub(crate) fn new(managed_vec: &'a ManagedVec<M, T>) -> Self {
        ManagedVecIterator {
            managed_vec,
            byte_index: 0,
            byte_limit: managed_vec.byte_len(),
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
        let next_byte_index = self.byte_index + T::PAYLOAD_SIZE;
        if next_byte_index > self.byte_limit {
            return None;
        }
        let result = T::from_byte_reader(self.type_manager(), |dest_slice| {
            let _ = self
                .managed_vec
                .buffer
                .load_slice(self.byte_index, dest_slice);
        });

        self.byte_index = next_byte_index;
        Some(result)
    }
}
