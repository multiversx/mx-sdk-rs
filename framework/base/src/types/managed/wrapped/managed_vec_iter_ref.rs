use core::marker::PhantomData;

use multiversx_sc_codec::{DecodeError, DecodeErrorHandler, TopDecodeMultiInput};

use crate::{
    api::{ErrorApi, ManagedTypeApi},
    types::{ManagedBuffer, ManagedType},
};

use super::{ManagedVec, ManagedVecItem, ManagedVecPayloadIterator};

impl<'a, M, T> IntoIterator for &'a ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    type Item = T::Ref<'a>;
    type IntoIter = ManagedVecRefIterator<'a, M, T>;
    fn into_iter(self) -> Self::IntoIter {
        ManagedVecRefIterator::new(self)
    }
}

pub struct ManagedVecRefIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    payload_iter: ManagedVecPayloadIterator<M, T::PAYLOAD>,
    _phantom: PhantomData<&'a ManagedVec<M, T>>,
}

impl<'a, M, T> ManagedVecRefIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    pub(crate) unsafe fn new_from_handle(vec_handle: M::ManagedBufferHandle) -> Self {
        unsafe {
            ManagedVecRefIterator {
                payload_iter: ManagedVecPayloadIterator::new(vec_handle),
                _phantom: PhantomData,
            }
        }
    }

    pub(crate) fn new(managed_vec: &'a ManagedVec<M, T>) -> Self {
        unsafe { ManagedVecRefIterator::new_from_handle(managed_vec.get_handle()) }
    }

    pub(crate) fn iter_is_empty(&self) -> bool {
        self.payload_iter.iter_is_empty()
    }
}

impl<'a, M, T> Iterator for ManagedVecRefIterator<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    type Item = T::Ref<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let payload = self.payload_iter.next()?;
        unsafe { Some(T::borrow_from_payload(&payload)) }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.payload_iter.size_hint()
    }
}

impl<M, T> ExactSizeIterator for ManagedVecRefIterator<'_, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
}

impl<M, T> DoubleEndedIterator for ManagedVecRefIterator<'_, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let payload = self.payload_iter.next_back()?;
        unsafe { Some(T::borrow_from_payload(&payload)) }
    }
}

impl<M, T> Clone for ManagedVecRefIterator<'_, M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    fn clone(&self) -> Self {
        unsafe {
            ManagedVecRefIterator {
                payload_iter: self.payload_iter.clone_iter(),
                _phantom: PhantomData,
            }
        }
    }
}

impl<A> TopDecodeMultiInput for ManagedVecRefIterator<'_, A, ManagedBuffer<A>>
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
            Ok(buffer.clone())
        } else {
            Err(h.handle_error(DecodeError::MULTI_TOO_FEW_ARGS))
        }
    }
}
