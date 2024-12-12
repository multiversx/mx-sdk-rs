use core::marker::PhantomData;

use unwrap_infallible::UnwrapInfallible;

use crate::codec::{TopDecodeMulti, TopDecodeMultiInput};

use crate::types::{ManagedBuffer, ManagedVec, ManagedVecOwnedIterator};
use crate::{
    api::{ErrorApi, ManagedTypeApi},
    io::{ArgErrorHandler, ArgId},
};

/// Iterator for `MultiValueEncoded` and `MultiValueEncodedCounted`.
///
/// Decodes items while it is iterating.
pub struct MultiValueEncodedIterator<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopDecodeMulti,
{
    data_loader: ManagedVecOwnedIterator<M, ManagedBuffer<M>>,
    _phantom: PhantomData<T>,
}

impl<M, T> MultiValueEncodedIterator<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopDecodeMulti,
{
    pub(crate) fn new(raw_buffers: ManagedVec<M, ManagedBuffer<M>>) -> Self {
        MultiValueEncodedIterator {
            data_loader: raw_buffers.into_iter(),
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
        if !self.data_loader.has_next() {
            return None;
        }

        let arg_id = ArgId::from(&b"var args"[..]);
        let h = ArgErrorHandler::<M>::from(arg_id);
        let result = T::multi_decode_or_handle_err(&mut self.data_loader, h).unwrap_infallible();
        Some(result)
    }
}
