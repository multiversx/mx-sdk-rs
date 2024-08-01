use crate::abi::{TypeAbi, TypeName};
use crate::types::ManagedBuffer;
use crate::{abi::TypeAbiFrom, api::ManagedTypeApi};
use multiversx_sc_codec::{
    DecodeError, DecodeErrorHandler, EncodeError, EncodeErrorHandler, NestedDecode,
    NestedDecodeInput, NestedEncode, NestedEncodeOutput, TryStaticCast,
};

/// A wrapper over a ManagedBuffer with different decode properties. It reads until the end of the buffer.
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct ManagedBufferReadToEnd<M: ManagedTypeApi> {
    pub(crate) buffer: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> ManagedBufferReadToEnd<M> {
    #[inline]
    pub fn new_from_buf(buf: ManagedBuffer<M>) -> Self {
        Self { buffer: buf }
    }

    #[inline]
    pub fn as_managed_buffer(&self) -> &ManagedBuffer<M> {
        &self.buffer
    }

    #[inline]
    pub fn into_managed_buffer(self) -> ManagedBuffer<M> {
        self.buffer
    }
}

impl<M: ManagedTypeApi> PartialEq for ManagedBufferReadToEnd<M> {
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }
}

impl<M> From<ManagedBuffer<M>> for ManagedBufferReadToEnd<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from(buf: ManagedBuffer<M>) -> Self {
        Self::new_from_buf(buf)
    }
}

impl<M> TypeAbiFrom<Self> for ManagedBufferReadToEnd<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&Self> for ManagedBufferReadToEnd<M> where M: ManagedTypeApi {}

impl<M: ManagedTypeApi> TypeAbi for ManagedBufferReadToEnd<M> {
    type Unmanaged = multiversx_sc_codec::Vec<u8>;

    fn type_name() -> TypeName {
        "bytes-read-to-end".into()
    }

    fn type_name_rust() -> TypeName {
        "ManagedBufferReadToEnd<$API>".into()
    }
}

impl<M: ManagedTypeApi> TryStaticCast for ManagedBufferReadToEnd<M> {}

impl<M: ManagedTypeApi> NestedDecode for ManagedBufferReadToEnd<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        if I::supports_specialized_type::<Self>() {
            input.read_specialized((), h)
        } else {
            Err(h.handle_error(DecodeError::UNSUPPORTED_OPERATION))
        }
    }
}

impl<M: ManagedTypeApi> NestedEncode for ManagedBufferReadToEnd<M> {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        if O::supports_specialized_type::<ManagedBuffer<M>>() {
            dest.push_specialized((), &self.buffer, h)
        } else {
            Err(h.handle_error(EncodeError::UNSUPPORTED_OPERATION))
        }
    }
}
