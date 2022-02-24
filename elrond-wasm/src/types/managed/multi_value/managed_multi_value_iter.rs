use core::marker::PhantomData;

use elrond_codec::{TopDecodeMulti, TopDecodeMultiInput};

use crate::{
    api::{ErrorApi, ManagedTypeApi},
    ArgErrorHandler, ArgId, ManagedResultArgLoader,
};

use super::ManagedMultiResultVec;

impl<M, T> IntoIterator for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopDecodeMulti,
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
    T: TopDecodeMulti,
{
    data_loader: ManagedResultArgLoader<M>,
    _phantom: PhantomData<T>,
}

impl<M, T> ManagedMultiResultVecIterator<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopDecodeMulti,
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
    T: TopDecodeMulti,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.data_loader.has_next() {
            let arg_id = ArgId::from(&b"var args"[..]);
            let h = ArgErrorHandler::<M>::from(arg_id);
            let Ok(result) = T::multi_decode_or_handle_err(&mut self.data_loader, h);
            Some(result)
        } else {
            None
        }
    }
}
