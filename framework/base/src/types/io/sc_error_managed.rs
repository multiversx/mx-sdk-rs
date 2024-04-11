use crate::codec::{EncodeErrorHandler, TopEncodeMulti, TopEncodeMultiOutput, TryStaticCast};
use alloc::{string::String, vec::Vec};
use core::mem;

use crate::{
    api::{EndpointFinishApi, ErrorApi, ErrorApiImpl, ManagedTypeApi},
    types::{heap::BoxedBytes, ManagedBuffer, ManagedType},
};

use super::SCError;

/// Smart contract error that can concatenate multiple message pieces.
/// The message is kept as a managed buffer in the VM.
pub struct ManagedSCError<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    buffer_handle: M::ManagedBufferHandle,
}

impl<'a, M> SCError for ManagedSCError<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    fn finish_err<FA: EndpointFinishApi>(&self) -> ! {
        M::error_api_impl().signal_error_from_buffer(self.buffer_handle.clone())
    }
}

impl<'a, M> TryStaticCast for ManagedSCError<'a, M> where M: ManagedTypeApi<'a> + ErrorApi {}

impl<'a, M> ManagedSCError<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    #[inline]
    pub fn new_empty() -> Self {
        ManagedSCError {
            buffer_handle: ManagedBuffer::<'a, M>::new().take_handle(),
        }
    }

    #[inline(always)]
    pub fn new_from_bytes(bytes: &[u8]) -> Self {
        ManagedSCError {
            buffer_handle: ManagedBuffer::<'a, M>::new_from_bytes(bytes).take_handle(),
        }
    }

    #[inline]
    pub fn append_bytes(&mut self, slice: &[u8]) {
        let mut buffer = ManagedBuffer::<'a, M>::from_handle(mem::take(&mut self.buffer_handle));
        buffer.append_bytes(slice);
        self.buffer_handle = buffer.take_handle();
    }

    #[inline]
    pub fn exit_now(self) -> ! {
        M::error_api_impl().signal_error_from_buffer(self.buffer_handle)
    }
}

impl<'a, M> From<&[u8]> for ManagedSCError<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    #[inline]
    fn from(message: &[u8]) -> Self {
        Self::new_from_bytes(message)
    }
}

impl<'a, M> From<BoxedBytes> for ManagedSCError<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    #[inline]
    fn from(message: BoxedBytes) -> Self {
        Self::new_from_bytes(message.as_slice())
    }
}

impl<'a, M> From<&str> for ManagedSCError<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    #[inline]
    fn from(message: &str) -> Self {
        Self::new_from_bytes(message.as_bytes())
    }
}

impl<'a, M> From<String> for ManagedSCError<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    #[inline]
    fn from(message: String) -> Self {
        Self::new_from_bytes(message.as_bytes())
    }
}

impl<'a, M> From<Vec<u8>> for ManagedSCError<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    #[inline]
    fn from(message: Vec<u8>) -> Self {
        Self::new_from_bytes(message.as_slice())
    }
}

impl<'a, M> From<ManagedBuffer<'a, M>> for ManagedSCError<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    #[inline]
    fn from(message: ManagedBuffer<'a, M>) -> Self {
        ManagedSCError { buffer_handle: message.take_handle() }
    }
}

impl<'a, M> TopEncodeMulti for ManagedSCError<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        output.push_multi_specialized(self, h)
    }
}
