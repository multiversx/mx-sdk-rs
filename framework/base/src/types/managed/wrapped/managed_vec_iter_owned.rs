use multiversx_sc_codec::{DecodeError, DecodeErrorHandler, TopDecodeMultiInput};

use crate::{
    api::{ErrorApi, ManagedTypeApi},
    types::{ManagedBuffer, ManagedType},
};

use super::{ManagedVec, ManagedVecItem, ManagedVecPayloadIterator};

impl<M, T> IntoIterator for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    type Item = T;
    type IntoIter = ManagedVecOwnedIterator<M, T>;
    fn into_iter(self) -> Self::IntoIter {
        ManagedVecOwnedIterator::new(self)
    }
}

pub struct ManagedVecOwnedIterator<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    payload_iter: ManagedVecPayloadIterator<M, T::PAYLOAD>,
}

impl<M, T> ManagedVecOwnedIterator<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    pub(crate) fn new(managed_vec: ManagedVec<M, T>) -> Self {
        unsafe {
            ManagedVecOwnedIterator {
                payload_iter: ManagedVecPayloadIterator::new(managed_vec.forget_into_handle()),
            }
        }
    }

    pub(crate) fn iter_is_empty(&self) -> bool {
        self.payload_iter.iter_is_empty()
    }
}

impl<M, T> Iterator for ManagedVecOwnedIterator<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let payload = self.payload_iter.next()?;
        // ok, because the iterator has ownership over the payload and no drop
        unsafe { Some(T::read_from_payload(&payload)) }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.payload_iter.size_hint()
    }
}

impl<M, T> ExactSizeIterator for ManagedVecOwnedIterator<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
}

impl<M, T> DoubleEndedIterator for ManagedVecOwnedIterator<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let payload = self.payload_iter.next_back()?;
        // ok, because the iterator has ownership over the payload and no drop
        unsafe { Some(T::read_from_payload(&payload)) }
    }
}

impl<A> TopDecodeMultiInput for ManagedVecOwnedIterator<A, ManagedBuffer<A>>
where
    A: ManagedTypeApi + ErrorApi,
{
    type ValueInput = ManagedBuffer<A>;

    fn has_next(&self) -> bool {
        !self.iter_is_empty()
    }

    fn next_value_input<H>(&mut self, h: H) -> Result<Self::ValueInput, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        if let Some(buffer) = self.next() {
            Ok(buffer)
        } else {
            Err(h.handle_error(DecodeError::MULTI_TOO_FEW_ARGS))
        }
    }
}
