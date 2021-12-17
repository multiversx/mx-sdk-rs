use core::marker::PhantomData;

use elrond_codec::{DecodeError, EncodeError, TopDecode, TopEncode};

use crate::{
    api::{ErrorApi, ErrorApiImpl, ManagedTypeApi, ManagedTypeErrorApi},
    err_msg,
    types::{AsManagedRef, BoxedBytes, ManagedBuffer, ManagedBytesTopDecodeInput, ManagedType},
};

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
        value.top_encode_or_exit(&mut result, (), top_encode_exit::<M>);
        result
    }

    pub fn top_encode_to_boxed_bytes<T: TopEncode>(&self, value: &T) -> BoxedBytes {
        let mut result = BoxedBytes::empty();
        value.top_encode_or_exit(&mut result, (), top_encode_exit::<M>);
        result
    }

    pub fn top_decode_from_managed_buffer<T: TopDecode>(&self, buffer: &ManagedBuffer<M>) -> T {
        T::top_decode_or_exit(buffer.as_managed_ref(), (), top_decode_exit::<M>)
    }

    pub fn top_decode_from_byte_slice<T: TopDecode>(&self, slice: &[u8]) -> T {
        let managed_input = ManagedBytesTopDecodeInput::<M>::new(BoxedBytes::from(slice));
        T::top_decode_or_exit(managed_input, (), top_decode_exit::<M>)
    }
}

#[inline(always)]
fn top_encode_exit<M>(_: (), encode_err: EncodeError) -> !
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    let mut message_buffer = ManagedBuffer::<M>::new_from_bytes(err_msg::SERIALIZER_ENCODE_ERROR);
    message_buffer.append_bytes(encode_err.message_bytes());
    M::error_api_impl().signal_error_from_buffer(message_buffer.get_raw_handle())
}

#[inline(always)]
fn top_decode_exit<M>(_: (), decode_err: DecodeError) -> !
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    let mut message_buffer = ManagedBuffer::<M>::new_from_bytes(err_msg::SERIALIZER_DECODE_ERROR);
    message_buffer.append_bytes(decode_err.message_bytes());
    M::error_api_impl().signal_error_from_buffer(message_buffer.get_raw_handle())
}
