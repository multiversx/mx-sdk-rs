use core::marker::PhantomData;

use crate::{
    api::{ErrorApi, ManagedTypeApi},
    ArgId, DynArg, DynArgInput, ManagedResultArgLoader,
};

use super::ManagedMultiResultVec;

impl<M, T> IntoIterator for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: DynArg,
{
    type Item = T;
    type IntoIter = ManagedMultiResultVecIterator<M, T>;
    fn into_iter(self) -> Self::IntoIter {
        ManagedMultiResultVecIterator::new(self)
    }
}

pub struct ManagedMultiResultVecIterator<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: DynArg,
{
    data_loader: ManagedResultArgLoader<M>,
    _phantom: PhantomData<T>,
}

impl<M, T> ManagedMultiResultVecIterator<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: DynArg,
{
    pub(crate) fn new(obj: ManagedMultiResultVec<M, T>) -> Self {
        ManagedMultiResultVecIterator {
            data_loader: ManagedResultArgLoader::new(obj.raw_buffers),
            _phantom: PhantomData,
        }
    }
}

impl<M, T> Iterator for ManagedMultiResultVecIterator<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: DynArg,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.data_loader.has_next() {
            let arg_id = ArgId::from(&b"var args"[..]);
            Some(T::dyn_load(&mut self.data_loader, arg_id))
        } else {
            None
        }
    }
}
