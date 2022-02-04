use super::ArgBuffer;
use crate::{
    api::{ErrorApi, ErrorApiImpl, Handle, ManagedTypeApi},
    err_msg,
    types::{ManagedBuffer, ManagedType, ManagedVec, ManagedVecRefIterator},
    DynArgOutput,
};
use alloc::vec::Vec;
use elrond_codec::{
    DecodeErrorHandler, EncodeError, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

#[derive(Debug)]
#[repr(transparent)]
pub struct ManagedArgBuffer<M>
where
    M: ManagedTypeApi + 'static,
{
    pub(crate) data: ManagedVec<M, ManagedBuffer<M>>,
}

impl<M: ManagedTypeApi> ManagedType<M> for ManagedArgBuffer<M>
where
    M: ManagedTypeApi + 'static,
{
    #[inline]
    fn from_raw_handle(handle: Handle) -> Self {
        ManagedArgBuffer {
            data: ManagedVec::from_raw_handle(handle),
        }
    }

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle {
        self.data.get_raw_handle()
    }

    fn transmute_from_handle_ref(handle_ref: &Handle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> ManagedArgBuffer<M>
where
    M: ManagedTypeApi + 'static,
{
    #[inline]
    pub fn new_empty() -> Self {
        ManagedArgBuffer {
            data: ManagedVec::new(),
        }
    }
}

impl<M, I> From<Vec<I>> for ManagedArgBuffer<M>
where
    M: ManagedTypeApi,
    I: Into<ManagedBuffer<M>>,
{
    fn from(v: Vec<I>) -> Self {
        ManagedArgBuffer { data: v.into() }
    }
}

impl<M> From<ManagedVec<M, ManagedBuffer<M>>> for ManagedArgBuffer<M>
where
    M: ManagedTypeApi,
{
    fn from(data: ManagedVec<M, ManagedBuffer<M>>) -> Self {
        ManagedArgBuffer { data }
    }
}

impl<M> ManagedArgBuffer<M>
where
    M: ManagedTypeApi + 'static,
{
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub fn push_arg_raw(&mut self, raw_arg: ManagedBuffer<M>) {
        self.data.push(raw_arg);
    }

    /// Concatenates 2 managed arg buffers. Consumes both arguments in the process.
    #[inline]
    #[must_use]
    pub fn concat(mut self, other: ManagedArgBuffer<M>) -> Self {
        self.data.append_vec(other.data);
        self
    }

    pub fn to_raw_args_vec(&self) -> Vec<Vec<u8>> {
        let mut v = Vec::new();
        for item in self.data.into_iter() {
            v.push(item.to_boxed_bytes().into_vec());
        }
        v
    }
}

impl<M> ManagedArgBuffer<M>
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    pub fn push_arg<T: TopEncode>(&mut self, arg: T) {
        let mut encoded_buffer = ManagedBuffer::new();
        arg.top_encode_or_exit(&mut encoded_buffer, (), managed_arg_buffer_push_exit::<M>);
        self.push_arg_raw(encoded_buffer);
    }
}

#[inline(always)]
fn managed_arg_buffer_push_exit<A>(_: (), encode_err: EncodeError) -> !
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    let mut message_buffer =
        ManagedBuffer::<A>::new_from_bytes(err_msg::CONTRACT_CALL_ENCODE_ERROR);
    message_buffer.append_bytes(encode_err.message_bytes());
    A::error_api_impl().signal_error_from_buffer(message_buffer.get_raw_handle())
}

impl<M: ManagedTypeApi> ManagedArgBuffer<M>
where
    M: ManagedTypeApi + 'static,
{
    pub fn to_legacy_arg_buffer(&self) -> ArgBuffer {
        let mut result = ArgBuffer::new();
        for m_arg in self.data.into_iter() {
            result.push_argument_bytes(m_arg.to_boxed_bytes().as_slice());
        }
        result
    }
}

impl<M: ManagedTypeApi> ManagedArgBuffer<M>
where
    M: ManagedTypeApi + 'static,
{
    pub fn raw_arg_iter(&self) -> ManagedVecRefIterator<M, ManagedBuffer<M>> {
        self.data.iter()
    }
}

impl<M: ManagedTypeApi> DynArgOutput for ManagedArgBuffer<M> {
    #[inline]
    fn push_single_arg<T: TopEncode>(&mut self, arg: T) {
        self.push_arg(arg)
    }
}

impl<M> TopEncode for ManagedArgBuffer<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.data.top_encode_or_handle_err(output, h)
    }
}

impl<M> NestedEncode for ManagedArgBuffer<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.data.dep_encode_or_handle_err(dest, h)
    }
}

impl<M> TopDecode for ManagedArgBuffer<M>
where
    M: ManagedTypeApi,
{
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(ManagedVec::top_decode_or_handle_err(input, h)?.into())
    }
}

impl<M> NestedDecode for ManagedArgBuffer<M>
where
    M: ManagedTypeApi,
{
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(ManagedVec::dep_decode_or_handle_err(input, h)?.into())
    }
}
