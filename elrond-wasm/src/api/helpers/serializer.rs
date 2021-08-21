use elrond_codec::{DecodeError, EncodeError, TopDecode, TopEncode};

use crate::{
    api::{ErrorApi, ManagedTypeApi},
    types::{BoxedBytes, ManagedBuffer, ManagedBytesTopDecodeInput},
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
    pub(crate) fn new(api: M) -> Self {
        ManagedSerializer { api }
    }

    pub fn top_encode_to_managed_buffer<T: TopEncode>(&self, value: &T) -> ManagedBuffer<M> {
        let mut result = ManagedBuffer::new_empty(self.api.clone());
        value.top_encode_or_exit(&mut result, self.api.clone(), top_encode_exit);
        result
    }

    pub fn top_encode_to_boxed_bytes<T: TopEncode>(&self, value: &T) -> BoxedBytes {
        let mut result = BoxedBytes::empty();
        value.top_encode_or_exit(&mut result, self.api.clone(), top_encode_exit);
        result
    }

    pub fn top_decode_from_managed_buffer<T: TopDecode>(&self, buffer: &ManagedBuffer<M>) -> T {
        T::top_decode_or_exit(buffer, self.api.clone(), top_decode_exit)
    }

    pub fn top_decode_from_byte_slice<T: TopDecode>(&self, slice: &[u8]) -> T {
        let managed_input =
            ManagedBytesTopDecodeInput::new(BoxedBytes::from(slice), self.api.clone());
        T::top_decode_or_exit(managed_input, self.api.clone(), top_decode_exit)
    }
}

#[inline(always)]
fn top_encode_exit<M>(api: M, encode_err: EncodeError) -> !
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    // TODO: error message
    api.signal_error(encode_err.message_bytes())
}

#[inline(always)]
fn top_decode_exit<M>(api: M, de_err: DecodeError) -> !
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    // TODO: error message
    api.signal_error(de_err.message_bytes())
}
