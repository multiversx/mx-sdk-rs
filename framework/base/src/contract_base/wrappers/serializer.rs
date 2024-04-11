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
pub struct ManagedSerializer<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi + 'static,
{
    _phantom: PhantomData<M>,
}

impl<'a, M> ManagedSerializer<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi + 'static,
{
    pub fn new() -> Self {
        ManagedSerializer {
            _phantom: PhantomData,
        }
    }

    pub fn top_encode_to_managed_buffer<T: TopEncode>(&self, value: &T) -> ManagedBuffer<'a, M> {
        let mut result = ManagedBuffer::new();
        let Ok(()) = value.top_encode_or_handle_err(
            &mut result,
            ExitCodecErrorHandler::<'a, M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
        );
        result
    }

    pub fn top_encode_to_boxed_bytes<T: TopEncode>(&self, value: &T) -> BoxedBytes {
        let mut result = BoxedBytes::empty();
        let Ok(()) = value.top_encode_or_handle_err(
            &mut result,
            ExitCodecErrorHandler::<'a, M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
        );
        result
    }

    pub fn top_decode_from_managed_buffer<T: TopDecode>(&self, buffer: &ManagedBuffer<'a, M>) -> T {
        self.top_decode_from_managed_buffer_custom_message(buffer, err_msg::SERIALIZER_DECODE_ERROR)
    }

    pub fn top_decode_from_managed_buffer_custom_message<T: TopDecode>(
        &self,
        buffer: &ManagedBuffer<'a, M>,
        error_message: &'static [u8],
    ) -> T {
        let Ok(value) = T::top_decode_or_handle_err(
            buffer.clone(), // TODO: remove clone
            ExitCodecErrorHandler::<'a, M>::from(error_message),
        );
        value
    }

    pub fn top_decode_from_byte_slice<T: TopDecode>(&self, slice: &[u8]) -> T {
        let Ok(value) = T::top_decode_or_handle_err(
            slice,
            ExitCodecErrorHandler::<'a, M>::from(err_msg::SERIALIZER_DECODE_ERROR),
        );
        value
    }
}

#[derive(Clone)]
pub struct ExitCodecErrorHandler<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    _phantom: PhantomData<M>,
    pub base_message: &'static [u8],
}

impl<'a, M> Copy for ExitCodecErrorHandler<'a, M> where M: ManagedTypeApi<'a> + ErrorApi {}

impl<'a, M> From<&'static [u8]> for ExitCodecErrorHandler<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    fn from(base_message: &'static [u8]) -> Self {
        ExitCodecErrorHandler {
            _phantom: PhantomData,
            base_message,
        }
    }
}

impl<'a, M> EncodeErrorHandler for ExitCodecErrorHandler<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    type HandledErr = !;

    fn handle_error(&self, err: EncodeError) -> Self::HandledErr {
        let mut message_buffer = ManagedBuffer::<'a, M>::new_from_bytes(self.base_message);
        message_buffer.append_bytes(err.message_bytes());
        M::error_api_impl().signal_error_from_buffer(message_buffer.take_handle())
    }
}

impl<'a, M> DecodeErrorHandler for ExitCodecErrorHandler<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    type HandledErr = !;

    fn handle_error(&self, err: DecodeError) -> Self::HandledErr {
        let mut message_buffer = ManagedBuffer::<'a, M>::new_from_bytes(self.base_message);
        message_buffer.append_bytes(err.message_bytes());
        M::error_api_impl().signal_error_from_buffer(message_buffer.take_handle())
    }
}
