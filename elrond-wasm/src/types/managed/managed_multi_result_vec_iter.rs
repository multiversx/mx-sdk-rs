use core::marker::PhantomData;

use elrond_codec::TopDecode;

use crate::{
    api::{ErrorApi, ManagedTypeApi},
    contract_base::ManagedSerializer,
};

use super::{ManagedBuffer, ManagedMultiResultVec, ManagedVecIterator};

impl<'a, M, T> IntoIterator for &'a ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopDecode,
{
    type Item = T;
    type IntoIter = ManagedMultiResultVecIterator<'a, M, T>;
    fn into_iter(self) -> Self::IntoIter {
        ManagedMultiResultVecIterator::new(self)
    }
}

pub struct ManagedMultiResultVecIterator<'a, M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopDecode,
{
    managed_vec_iter: ManagedVecIterator<'a, M, ManagedBuffer<M>>,
    _phantom: PhantomData<T>,
}

impl<'a, M, T> ManagedMultiResultVecIterator<'a, M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopDecode,
{
    pub(crate) fn new(obj: &'a ManagedMultiResultVec<M, T>) -> Self {
        ManagedMultiResultVecIterator {
            managed_vec_iter: obj.raw_buffers.into_iter(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, M, T> Iterator for ManagedMultiResultVecIterator<'a, M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopDecode,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let serializer = ManagedSerializer::new(self.managed_vec_iter.type_manager());
        self.managed_vec_iter
            .next()
            .map(|managed_buffer| serializer.top_decode_from_managed_buffer(&managed_buffer))
    }
}
