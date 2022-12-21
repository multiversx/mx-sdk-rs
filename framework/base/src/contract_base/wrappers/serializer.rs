use core::marker::PhantomData;

use crate::codec::{
    DecodeError, DecodeErrorHandler, EncodeError, EncodeErrorHandler, TopDecode, TopEncode,
};

use crate::{
    api::{ErrorApi, ErrorApiImpl, ManagedTypeApi},
    err_msg,
    types::{heap::BoxedBytes, ManagedBuffer, ManagedType},
};

#[derive(Default)]
pub struct ManagedSerializer<M>
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    _phantom: PhantomData<M>,
}

impl<M> ManagedSerializer<M>
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    pub fn new() -> Self {
        ManagedSerializer {
            _phantom: PhantomData,
        }
    }

    pub fn top_encode_to_managed_buffer<T: TopEncode>(&self, value: &T) -> ManagedBuffer<M> {
        let mut result = ManagedBuffer::new();
        let Ok(()) = value.top_encode_or_handle_err(
            &mut result,
            ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
        );
        result
    }

    pub fn top_encode_to_boxed_bytes<T: TopEncode>(&self, value: &T) -> BoxedBytes {
        let mut result = BoxedBytes::empty();
        let Ok(()) = value.top_encode_or_handle_err(
            &mut result,
            ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
        );
        result
    }

    pub fn top_decode_from_managed_buffer<T: TopDecode>(&self, buffer: &ManagedBuffer<M>) -> T {
        self.top_decode_from_managed_buffer_custom_message(buffer, err_msg::SERIALIZER_DECODE_ERROR)
    }

    pub fn top_decode_from_managed_buffer_custom_message<T: TopDecode>(
        &self,
        buffer: &ManagedBuffer<M>,
        error_message: &'static [u8],
    ) -> T {
        let Ok(value) = T::top_decode_or_handle_err(
            buffer.clone(), // TODO: remove clone
            ExitCodecErrorHandler::<M>::from(error_message),
        );
        value
    }

    pub fn top_decode_from_byte_slice<T: TopDecode>(&self, slice: &[u8]) -> T {
        let Ok(value) = T::top_decode_or_handle_err(
            slice,
            ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_DECODE_ERROR),
        );
        value
    }
}

#[derive(Clone)]
pub struct ExitCodecErrorHandler<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    _phantom: PhantomData<M>,
    pub base_message: &'static [u8],
}

impl<M> Copy for ExitCodecErrorHandler<M> where M: ManagedTypeApi + ErrorApi {}

impl<M> From<&'static [u8]> for ExitCodecErrorHandler<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    fn from(base_message: &'static [u8]) -> Self {
        ExitCodecErrorHandler {
            _phantom: PhantomData,
            base_message,
        }
    }
}

impl<M> EncodeErrorHandler for ExitCodecErrorHandler<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    type HandledErr = !;

    fn handle_error(&self, err: EncodeError) -> Self::HandledErr {
        let mut message_buffer = ManagedBuffer::<M>::new_from_bytes(self.base_message);
        message_buffer.append_bytes(err.message_bytes());
        M::error_api_impl().signal_error_from_buffer(message_buffer.get_handle())
    }
}

impl<M> DecodeErrorHandler for ExitCodecErrorHandler<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    type HandledErr = !;

    fn handle_error(&self, err: DecodeError) -> Self::HandledErr {
        let mut message_buffer = ManagedBuffer::<M>::new_from_bytes(self.base_message);
        message_buffer.append_bytes(err.message_bytes());
        M::error_api_impl().signal_error_from_buffer(message_buffer.get_handle())
    }
}
