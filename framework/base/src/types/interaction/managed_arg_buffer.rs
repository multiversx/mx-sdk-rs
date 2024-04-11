use crate::{
    abi::{TypeAbi, TypeName},
    api::{ErrorApi, ManagedTypeApi},
    codec::{
        DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
        NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeMultiOutput,
        TopEncodeOutput,
    },
    contract_base::ExitCodecErrorHandler,
    err_msg,
    types::{
        heap::ArgBuffer, ManagedBuffer, ManagedBufferNestedDecodeInput, ManagedRef, ManagedType,
        ManagedVec, ManagedVecRefIterator, MultiValueEncoded,
    },
};
use alloc::vec::Vec;
use multiversx_sc_codec::TopEncodeMulti;

#[derive(Debug, Default, Clone)]
#[repr(transparent)]
pub struct ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a> + 'static,
{
    pub(crate) data: ManagedVec<'a, M, ManagedBuffer<'a, M>>,
}

impl<'a, M: ManagedTypeApi<'a>> ManagedType<'a, M> for ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a> + 'static,
{
    type OwnHandle = M::ManagedBufferHandle;

    #[inline]
    fn from_handle(handle: M::ManagedBufferHandle) -> Self {
        ManagedArgBuffer {
            data: ManagedVec::from_handle(handle),
        }
    }

    unsafe fn get_handle(&self) -> M::ManagedBufferHandle {
        self.data.get_handle()
    }

    fn take_handle(self) -> Self::OwnHandle {
        self.data.take_handle()
    }

    fn transmute_from_handle_ref(handle_ref: &M::ManagedBufferHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<'a, M: ManagedTypeApi<'a>> ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a> + 'static,
{
    #[inline]
    pub fn new() -> Self {
        ManagedArgBuffer {
            data: ManagedVec::new(),
        }
    }
}

impl<'a, M, I> From<Vec<I>> for ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a>,
    I: Into<ManagedBuffer<'a, M>>,
{
    fn from(v: Vec<I>) -> Self {
        ManagedArgBuffer { data: v.into() }
    }
}

impl<'a, M, I> From<&[I]> for ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a>,
    I: Into<ManagedBuffer<'a, M>> + TopEncode,
{
    fn from(arguments: &[I]) -> Self {
        let mut arg_buffer = Self::new();
        for arg in arguments {
            arg_buffer.push_arg(arg);
        }
        arg_buffer
    }
}

impl<'a, M> From<ArgBuffer> for ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    fn from(arg_buffer: ArgBuffer) -> Self {
        let mut data = ManagedVec::new();
        for arg in arg_buffer.arg_data().iter() {
            data.push(ManagedBuffer::new_from_bytes(&[*arg]));
        }

        ManagedArgBuffer { data }
    }
}

impl<'a, M> From<&ArgBuffer> for ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    fn from(arg_buffer: &ArgBuffer) -> Self {
        let mut data = ManagedVec::new();
        for arg in arg_buffer.arg_data().iter() {
            data.push(ManagedBuffer::new_from_bytes(&[*arg]));
        }

        ManagedArgBuffer { data }
    }
}

impl<'a, M> From<ManagedVec<'a, M, ManagedBuffer<'a, M>>> for ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    fn from(data: ManagedVec<'a, M, ManagedBuffer<'a, M>>) -> Self {
        ManagedArgBuffer { data }
    }
}

impl<'a, M> ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a> + 'static,
{
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get(&self, index: usize) -> ManagedRef<'_, M, ManagedBuffer<'a, M>> {
        self.data.get(index)
    }

    #[inline]
    pub fn push_arg_raw(&mut self, raw_arg: ManagedBuffer<'a, M>) {
        self.data.push(raw_arg);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Concatenates 2 managed arg buffers. Consumes both arguments in the process.
    #[inline]
    #[must_use]
    pub fn concat(mut self, other: ManagedArgBuffer<'a, M>) -> Self {
        self.data.append_vec(other.data);
        self
    }

    #[cfg(feature = "alloc")]
    pub fn to_raw_args_vec(&self) -> Vec<Vec<u8>> {
        let mut v = Vec::new();
        for item in self.data.into_iter() {
            v.push(item.to_boxed_bytes().into_vec());
        }
        v
    }

    pub fn into_multi_value_encoded(self) -> MultiValueEncoded<'a, M, ManagedBuffer<'a, M>> {
        self.data.into()
    }

    pub fn into_vec_of_buffers(self) -> ManagedVec<'a, M, ManagedBuffer<'a, M>> {
        self.data
    }
}

impl<'a, M> ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi + 'static,
{
    pub fn push_arg<T: TopEncode>(&mut self, arg: T) {
        let mut encoded_buffer = ManagedBuffer::new();
        let Ok(()) = arg.top_encode_or_handle_err(
            &mut encoded_buffer,
            ExitCodecErrorHandler::<'a, M>::from(err_msg::CONTRACT_CALL_ENCODE_ERROR),
        );
        self.push_arg_raw(encoded_buffer);
    }

    pub fn push_multi_arg<T: TopEncodeMulti>(&mut self, arg: &T) {
        let h = ExitCodecErrorHandler::<'a, M>::from(err_msg::CONTRACT_CALL_ENCODE_ERROR);
        let Ok(()) = arg.multi_encode_or_handle_err(self, h);
    }
}

impl<'a, M: ManagedTypeApi<'a>> ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a> + 'static,
{
    pub fn raw_arg_iter(&self) -> ManagedVecRefIterator<'a, M, ManagedBuffer<'a, M>> {
        self.data.iter()
    }
}

impl<'a, M> TopEncodeMultiOutput for ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    fn push_single_value<T, H>(&mut self, arg: &T, h: H) -> Result<(), H::HandledErr>
    where
        T: TopEncode,
        H: EncodeErrorHandler,
    {
        self.data.push_single_value(arg, h)
    }
}

impl<'a, M> TopEncode for ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a>,
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

impl<'a, M> NestedEncode for ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a>,
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

impl<'a, M> TopDecode for ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(ManagedVec::top_decode_or_handle_err(input, h)?.into())
    }
}

impl<'a, M> NestedDecode for ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(ManagedVec::dep_decode_or_handle_err(input, h)?.into())
    }
}

impl<'a, M> ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi + 'static,
{
    /// Serializes itself into a managed buffer without allocating a new handle.
    /// Any data lying in the target buffer is overwritten.
    pub fn serialize_overwrite(&self, dest: &mut ManagedBuffer<'a, M>) {
        dest.overwrite(&[]);
        let h = ExitCodecErrorHandler::<'a, M>::from(err_msg::SERIALIZER_ENCODE_ERROR);
        let Ok(()) = self.top_encode_or_handle_err(dest, h);
    }

    /// Deserializes self from a managed buffer in-place, without creating a new handle.
    /// Any data lying in self is overwritten.
    pub fn deserialize_overwrite(&mut self, source: ManagedBuffer<'a, M>) {
        let h = ExitCodecErrorHandler::<'a, M>::from(err_msg::SERIALIZER_DECODE_ERROR);
        self.clear();
        let mut nested_de_input = ManagedBufferNestedDecodeInput::new(source);
        while nested_de_input.remaining_len() > 0 {
            let Ok(item) = ManagedBuffer::dep_decode_or_handle_err(&mut nested_de_input, h);
            self.push_arg_raw(item);
        }
    }
}

impl<'a, M> TypeAbi for ManagedArgBuffer<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    /// It is semantically equivalent to any list of `T`.
    fn type_name() -> TypeName {
        <&[ManagedBuffer<'a, M>] as TypeAbi>::type_name()
    }
}
