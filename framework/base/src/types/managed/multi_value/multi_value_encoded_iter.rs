use core::marker::PhantomData;

use unwrap_infallible::UnwrapInfallible;

use crate::codec::{TopDecodeMulti, TopDecodeMultiInput};

use crate::types::{ManagedBuffer, ManagedVec};
use crate::{
    api::{ErrorApi, ManagedTypeApi},
    io::{ArgErrorHandler, ArgId, ManagedResultArgLoader},
};

/// Iterator for `MultiValueEncoded` and `MultiValueEncodedCounted`.
///
/// Decodes items while it is iterating.
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
    pub(crate) fn new(raw_buffers: ManagedVec<M, ManagedBuffer<M>>) -> Self {
        MultiValueEncodedIterator {
            data_loader: ManagedResultArgLoader::new(raw_buffers),
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
            let result =
                T::multi_decode_or_handle_err(&mut self.data_loader, h).unwrap_infallible();
            Some(result)
        } else {
            None
        }
    }
}
