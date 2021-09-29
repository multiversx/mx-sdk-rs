use super::{ManagedBuffer, ManagedDefault, ManagedType};
use crate::{
    abi::TypeAbi,
    api::{Handle, ManagedTypeApi},
};
use alloc::string::String;
use elrond_codec::{
    DecodeError, EncodeError, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, TryStaticCast,
};

const DECODE_ERROR_BAD_LENGTH: &[u8] = b"bad array length";

/// A list of items that lives inside a managed buffer.
/// Items can be either stored there in full (e.g. `u32`),
/// or just via handle (e.g. `BigUint<M>`).
#[derive(Clone, Debug)]
pub struct ManagedByteArray<M, const N: usize>
where
    M: ManagedTypeApi,
{
    pub(crate) buffer: ManagedBuffer<M>,
}

impl<M, const N: usize> ManagedType<M> for ManagedByteArray<M, N>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from_raw_handle(api: M, handle: Handle) -> Self {
        ManagedByteArray {
            buffer: ManagedBuffer::from_raw_handle(api, handle),
        }
    }

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle {
        self.buffer.get_raw_handle()
    }

    #[inline]
    fn type_manager(&self) -> M {
        self.buffer.type_manager()
    }
}

impl<M, const N: usize> ManagedDefault<M> for ManagedByteArray<M, N>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn managed_default(api: M) -> Self {
        Self::new_from_bytes(api, &[0u8; N])
    }
}

impl<M, const N: usize> ManagedByteArray<M, N>
where
    M: ManagedTypeApi,
{
    #[inline]
    pub fn new_from_bytes(api: M, bytes: &[u8; N]) -> Self {
        ManagedByteArray {
            buffer: ManagedBuffer::new_from_bytes(api, &bytes[..]),
        }
    }

    /// Number of items.
    #[inline]
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn as_managed_buffer(&self) -> &ManagedBuffer<M> {
        &self.buffer
    }
}

impl<M, const N: usize> PartialEq for ManagedByteArray<M, N>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }
}

impl<M, const N: usize> Eq for ManagedByteArray<M, N> where M: ManagedTypeApi {}

impl<M, const N: usize> TopEncode for ManagedByteArray<M, N>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.buffer.top_encode(output)
    }
}

impl<M, const N: usize> TopDecode for ManagedByteArray<M, N>
where
    M: ManagedTypeApi,
{
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        let buffer = ManagedBuffer::top_decode(input)?;
        if buffer.len() != N {
            return Err(DecodeError::from(DECODE_ERROR_BAD_LENGTH));
        }
        Ok(ManagedByteArray { buffer })
    }
}

#[derive(Clone)]
pub(crate) struct ManagedBufferSizeContext(pub usize);

impl TryStaticCast for ManagedBufferSizeContext {}

impl<M, const N: usize> NestedEncode for ManagedByteArray<M, N>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        dest.push_specialized(ManagedBufferSizeContext(N), &self.buffer, |else_output| {
            else_output.write(self.buffer.to_boxed_bytes().as_slice());
            Ok(())
        })
    }
}

impl<M, const N: usize> NestedDecode for ManagedByteArray<M, N>
where
    M: ManagedTypeApi,
{
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        let buffer: ManagedBuffer<M> = input
            .read_specialized(ManagedBufferSizeContext(N), |_| {
                Err(DecodeError::UNSUPPORTED_OPERATION)
            })?;
        Ok(ManagedByteArray { buffer })
    }
}

impl<M, const N: usize> TypeAbi for ManagedByteArray<M, N>
where
    M: ManagedTypeApi,
{
    /// It is semantically equivalent to `[u8; N]`.
    fn type_name() -> String {
        <&[u8; N] as TypeAbi>::type_name()
    }
}
