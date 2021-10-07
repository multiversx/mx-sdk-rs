use alloc::{string::String, vec::Vec};

use crate::{
    api::{EndpointFinishApi, ErrorApi, ManagedTypeApi},
    types::{BoxedBytes, ManagedBuffer, ManagedFrom, ManagedType},
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
    fn finish_err<FA: EndpointFinishApi>(&self, api: FA) -> ! {
        api.signal_error_from_buffer(self.buffer.get_raw_handle())
    }
}

impl<M> ManagedSCError<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    #[inline]
    pub fn new_empty(api: M) -> Self {
        ManagedSCError {
            buffer: ManagedBuffer::new(api),
        }
    }

    #[inline(always)]
    pub fn new_from_bytes(api: M, bytes: &[u8]) -> Self {
        ManagedSCError {
            buffer: ManagedBuffer::new_from_bytes(api, bytes),
        }
    }

    #[inline]
    pub fn append_bytes(&mut self, slice: &[u8]) {
        self.buffer.append_bytes(slice)
    }

    #[inline]
    pub fn exit_now(&self) -> ! {
        self.buffer
            .api
            .signal_error_from_buffer(self.buffer.get_raw_handle())
    }
}

impl<M> ManagedFrom<M, &[u8]> for ManagedSCError<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn managed_from(api: M, message: &[u8]) -> Self {
        Self::new_from_bytes(api, message)
    }
}

impl<M> ManagedFrom<M, BoxedBytes> for ManagedSCError<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn managed_from(api: M, message: BoxedBytes) -> Self {
        Self::new_from_bytes(api, message.as_slice())
    }
}

impl<M> ManagedFrom<M, &str> for ManagedSCError<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn managed_from(api: M, message: &str) -> Self {
        Self::new_from_bytes(api, message.as_bytes())
    }
}

impl<M> ManagedFrom<M, String> for ManagedSCError<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn managed_from(api: M, message: String) -> Self {
        Self::new_from_bytes(api, message.as_bytes())
    }
}

impl<M> ManagedFrom<M, Vec<u8>> for ManagedSCError<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn managed_from(api: M, message: Vec<u8>) -> Self {
        Self::new_from_bytes(api, message.as_slice())
    }
}

impl<M> From<ManagedBuffer<M>> for ManagedSCError<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from(message: ManagedBuffer<M>) -> Self {
        ManagedSCError { buffer: message }
    }
}
