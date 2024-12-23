use core::marker::PhantomData;

use crate::api::{ManagedBufferApiImpl, ManagedTypeApi};

use super::ManagedVecItemPayload;

pub struct ManagedVecPayloadIterator<M, P>
where
    M: ManagedTypeApi,
    P: ManagedVecItemPayload,
{
    vec_handle: M::ManagedBufferHandle,
    byte_start: usize,
    byte_end: usize,
    _phantom: PhantomData<P>,
}

impl<M, P> ManagedVecPayloadIterator<M, P>
where
    M: ManagedTypeApi,
    P: ManagedVecItemPayload,
{
    /// Unsafe because it works with the managed vec handle directly, so does not take ownership into account.
    pub(crate) unsafe fn new(vec_handle: M::ManagedBufferHandle) -> Self {
        let byte_end = M::managed_type_impl().mb_len(vec_handle.clone());
        ManagedVecPayloadIterator {
            vec_handle,
            byte_start: 0,
            byte_end,
            _phantom: PhantomData,
        }
    }

    /// Unsafe because it works with the managed vec handle directly, so does not take ownership into account.
    pub(super) unsafe fn clone_iter(&self) -> Self {
        ManagedVecPayloadIterator {
            vec_handle: self.vec_handle.clone(),
            byte_start: self.byte_start,
            byte_end: self.byte_end,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn remaining_count(&self) -> usize {
        (self.byte_end - self.byte_start) / P::payload_size()
    }

    /// TODO: can be replaced with ExactSizeIterator::is_empty once it's stabilized
    pub(crate) fn iter_is_empty(&self) -> bool {
        self.byte_start >= self.byte_end
    }
}

impl<M, P> Iterator for ManagedVecPayloadIterator<M, P>
where
    M: ManagedTypeApi,
    P: ManagedVecItemPayload,
{
    type Item = P;

    fn next(&mut self) -> Option<P> {
        if self.iter_is_empty() {
            return None;
        }
        let next_byte_start = self.byte_start + P::payload_size();

        let mut payload = P::new_buffer();
        let _ = M::managed_type_impl().mb_load_slice(
            self.vec_handle.clone(),
            self.byte_start,
            payload.payload_slice_mut(),
        );

        self.byte_start = next_byte_start;
        Some(payload)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining_count();
        (remaining, Some(remaining))
    }
}

impl<M, P> ExactSizeIterator for ManagedVecPayloadIterator<M, P>
where
    M: ManagedTypeApi,
    P: ManagedVecItemPayload,
{
    fn len(&self) -> usize {
        self.remaining_count()
    }
}

impl<M, P> DoubleEndedIterator for ManagedVecPayloadIterator<M, P>
where
    M: ManagedTypeApi,
    P: ManagedVecItemPayload,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.iter_is_empty() {
            return None;
        }
        self.byte_end -= P::payload_size();

        let mut payload = P::new_buffer();
        let _ = M::managed_type_impl().mb_load_slice(
            self.vec_handle.clone(),
            self.byte_end,
            payload.payload_slice_mut(),
        );

        Some(payload)
    }
}
