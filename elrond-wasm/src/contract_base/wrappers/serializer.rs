use elrond_codec::{DecodeError, EncodeError, TopDecode, TopEncode};

use crate::{
    api::{ErrorApi, ManagedTypeApi},
    err_msg,
    types::{AsManagedRef, BoxedBytes, ManagedBuffer, ManagedBytesTopDecodeInput, ManagedType},
};

pub struct ManagedSerializer<M>
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    api: M,
}

impl<M> ManagedSerializer<M>
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    pub fn new(api: M) -> Self {
        ManagedSerializer { api }
    }

    pub fn top_encode_to_managed_buffer<T: TopEncode>(&self, value: &T) -> ManagedBuffer<M> {
        let mut result = ManagedBuffer::new();
        value.top_encode_or_exit(&mut result, self.api.clone(), top_encode_exit);
        result
    }

    pub fn top_encode_to_boxed_bytes<T: TopEncode>(&self, value: &T) -> BoxedBytes {
        let mut result = BoxedBytes::empty();
        value.top_encode_or_exit(&mut result, self.api.clone(), top_encode_exit);
        result
    }

    pub fn top_decode_from_managed_buffer<T: TopDecode>(&self, buffer: &ManagedBuffer<M>) -> T {
        T::top_decode_or_exit(buffer.as_managed_ref(), self.api.clone(), top_decode_exit)
    }

    pub fn top_decode_from_byte_slice<T: TopDecode>(&self, slice: &[u8]) -> T {
        let managed_input =
            ManagedBytesTopDecodeInput::new(self.api.clone(), BoxedBytes::from(slice));
        T::top_decode_or_exit(managed_input, self.api.clone(), top_decode_exit)
    }
}

#[inline(always)]
fn top_encode_exit<M>(api: M, encode_err: EncodeError) -> !
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    let mut message_buffer = ManagedBuffer::<M>::new_from_bytes(err_msg::SERIALIZER_ENCODE_ERROR);
    message_buffer.append_bytes(encode_err.message_bytes());
    api.signal_error_from_buffer(message_buffer.get_raw_handle())
}

#[inline(always)]
fn top_decode_exit<M>(api: M, decode_err: DecodeError) -> !
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    let mut message_buffer = ManagedBuffer::<M>::new_from_bytes(err_msg::SERIALIZER_DECODE_ERROR);
    message_buffer.append_bytes(decode_err.message_bytes());
    api.signal_error_from_buffer(message_buffer.get_raw_handle())
}
