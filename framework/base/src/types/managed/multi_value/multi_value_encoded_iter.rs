use core::marker::PhantomData;

use crate::codec::{TopDecodeMulti, TopDecodeMultiInput};

use crate::{
    api::{ErrorApi, ManagedTypeApi},
    io::{ArgErrorHandler, ArgId, ManagedResultArgLoader},
};

use super::MultiValueEncoded;

impl<M, T> IntoIterator for MultiValueEncoded<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopDecodeMulti,
{
    type Item = T;
    type IntoIter = MultiValueEncodedIterator<M, T>;
    fn into_iter(self) -> Self::IntoIter {
        MultiValueEncodedIterator::new(self)
    }
}

pub struct MultiValueEncodedIterator<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopDecodeMulti,
{
    data_loader: ManagedResultArgLoader<M>,
    _phantom: PhantomData<T>,
}

impl<M, T> MultiValueEncodedIterator<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopDecodeMulti,
{
    pub(crate) fn new(obj: MultiValueEncoded<M, T>) -> Self {
        MultiValueEncodedIterator {
            data_loader: ManagedResultArgLoader::new(obj.raw_buffers),
            _phantom: PhantomData,
        }
    }
}

impl<M, T> Iterator for MultiValueEncodedIterator<M, T>
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
