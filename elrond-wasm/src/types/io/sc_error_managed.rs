use alloc::{string::String, vec::Vec};

use crate::{
    api::{EndpointFinishApi, ErrorApi, ErrorApiImpl, ManagedTypeApi},
    types::{BoxedBytes, ManagedBuffer, ManagedType},
};

use super::SCError;

/// Smart contract error that can concatenate multiple message pieces.
/// The message is kept as a managed buffer in the VM.
pub struct ManagedSCError<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    buffer: ManagedBuffer<M>,
}

impl<M> SCError for ManagedSCError<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    fn finish_err<FA: EndpointFinishApi>(&self) -> ! {
        M::error_api_impl().signal_error_from_buffer(self.buffer.get_raw_handle())
    }
}

impl<M> ManagedSCError<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    #[inline]
    pub fn new_empty() -> Self {
        ManagedSCError {
            buffer: ManagedBuffer::new(),
        }
    }

    #[inline(always)]
    pub fn new_from_bytes(bytes: &[u8]) -> Self {
        ManagedSCError {
            buffer: ManagedBuffer::new_from_bytes(bytes),
        }
    }

    #[inline]
    pub fn append_bytes(&mut self, slice: &[u8]) {
        self.buffer.append_bytes(slice)
    }

    #[inline]
    pub fn exit_now(&self) -> ! {
        M::error_api_impl().signal_error_from_buffer(self.buffer.get_raw_handle())
    }
}

impl<M> From<&[u8]> for ManagedSCError<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    #[inline]
    fn from(message: &[u8]) -> Self {
        Self::new_from_bytes(message)
    }
}

impl<M> From<BoxedBytes> for ManagedSCError<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    #[inline]
    fn from(message: BoxedBytes) -> Self {
        Self::new_from_bytes(message.as_slice())
    }
}

impl<M> From<&str> for ManagedSCError<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    #[inline]
    fn from(message: &str) -> Self {
        Self::new_from_bytes(message.as_bytes())
    }
}

impl<M> From<String> for ManagedSCError<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    #[inline]
    fn from(message: String) -> Self {
        Self::new_from_bytes(message.as_bytes())
    }
}

impl<M> From<Vec<u8>> for ManagedSCError<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    #[inline]
    fn from(message: Vec<u8>) -> Self {
        Self::new_from_bytes(message.as_slice())
    }
}

impl<M> From<ManagedBuffer<M>> for ManagedSCError<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    #[inline]
    fn from(message: ManagedBuffer<M>) -> Self {
        ManagedSCError { buffer: message }
    }
}
