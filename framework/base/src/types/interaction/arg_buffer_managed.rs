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

#[derive(Debug, Default, Clone)]
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
    type OwnHandle = M::ManagedBufferHandle;

    #[inline]
    fn from_handle(handle: M::ManagedBufferHandle) -> Self {
        ManagedArgBuffer {
            data: ManagedVec::from_handle(handle),
        }
    }

    fn get_handle(&self) -> M::ManagedBufferHandle {
        self.data.get_handle()
    }

    fn transmute_from_handle_ref(handle_ref: &M::ManagedBufferHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> ManagedArgBuffer<M>
where
    M: ManagedTypeApi + 'static,
{
    #[inline]
    pub fn new() -> Self {
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

impl<M, I> From<&[I]> for ManagedArgBuffer<M>
where
    M: ManagedTypeApi,
    I: Into<ManagedBuffer<M>> + TopEncode,
{
    fn from(arguments: &[I]) -> Self {
        let mut arg_buffer = Self::new();
        for arg in arguments {
            arg_buffer.push_arg(arg);
        }
        arg_buffer
    }
}

impl<M> From<ArgBuffer> for ManagedArgBuffer<M>
where
    M: ManagedTypeApi,
{
    fn from(arg_buffer: ArgBuffer) -> Self {
        let mut data = ManagedVec::new();
        for arg in arg_buffer.arg_data().iter() {
            data.push(ManagedBuffer::new_from_bytes(&[*arg]));
        }

        ManagedArgBuffer { data }
    }
}

impl<M> From<&ArgBuffer> for ManagedArgBuffer<M>
where
    M: ManagedTypeApi,
{
    fn from(arg_buffer: &ArgBuffer) -> Self {
        let mut data = ManagedVec::new();
        for arg in arg_buffer.arg_data().iter() {
            data.push(ManagedBuffer::new_from_bytes(&[*arg]));
        }

        ManagedArgBuffer { data }
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

    pub fn get(&self, index: usize) -> ManagedRef<'_, M, ManagedBuffer<M>> {
        self.data.get(index)
    }

    #[inline]
    pub fn push_arg_raw(&mut self, raw_arg: ManagedBuffer<M>) {
        self.data.push(raw_arg);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Concatenates 2 managed arg buffers. Consumes both arguments in the process.
    #[inline]
    #[must_use]
    pub fn concat(mut self, other: ManagedArgBuffer<M>) -> Self {
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

    pub fn into_multi_value_encoded(self) -> MultiValueEncoded<M, ManagedBuffer<M>> {
        self.data.into()
    }
}

impl<M> ManagedArgBuffer<M>
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    pub fn push_arg<T: TopEncode>(&mut self, arg: T) {
        let mut encoded_buffer = ManagedBuffer::new();
        let Ok(()) = arg.top_encode_or_handle_err(
            &mut encoded_buffer,
            ExitCodecErrorHandler::<M>::from(err_msg::CONTRACT_CALL_ENCODE_ERROR),
        );
        self.push_arg_raw(encoded_buffer);
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

impl<M> TopEncodeMultiOutput for ManagedArgBuffer<M>
where
    M: ManagedTypeApi,
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

impl<M> ManagedArgBuffer<M>
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    /// Serializes itself into a managed buffer without allocating a new handle.
    /// Any data lying in the target buffer is overwritten.
    pub fn serialize_overwrite(&self, dest: &mut ManagedBuffer<M>) {
        dest.overwrite(&[]);
        let h = ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_ENCODE_ERROR);
        let Ok(()) = self.top_encode_or_handle_err(dest, h);
    }

    /// Deserializes self from a managed buffer in-place, without creating a new handle.
    /// Any data lying in self is overwritten.
    pub fn deserialize_overwrite(&mut self, source: ManagedBuffer<M>) {
        let h = ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_DECODE_ERROR);
        self.clear();
        let mut nested_de_input = ManagedBufferNestedDecodeInput::new(source);
        while nested_de_input.remaining_len() > 0 {
            let Ok(item) = ManagedBuffer::dep_decode_or_handle_err(&mut nested_de_input, h);
            self.push_arg_raw(item);
        }
    }
}

impl<M> TypeAbi for ManagedArgBuffer<M>
where
    M: ManagedTypeApi,
{
    /// It is semantically equivalent to any list of `T`.
    fn type_name() -> TypeName {
        <&[ManagedBuffer<M>] as TypeAbi>::type_name()
    }
}
