use core::convert::TryFrom;

use crate::{
    abi::{TypeAbi, TypeName},
    api::ManagedTypeApi,
    codec::{
        DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
        NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
        TryStaticCast,
    },
    formatter::{hex_util::encode_bytes_as_hex, FormatByteReceiver, SCLowerHex},
    types::{ManagedBuffer, ManagedType},
};

const DECODE_ERROR_BAD_LENGTH: &str = "bad array length";

/// A list of items that lives inside a managed buffer.
/// Items can be either stored there in full (e.g. `u32`),
/// or just via handle (e.g. `BigUint<'a, M>`).
#[repr(transparent)]
#[derive(Clone)]
pub struct ManagedByteArray<'a, M, const N: usize>
where
    M: ManagedTypeApi<'a>,
{
    pub(crate) buffer: ManagedBuffer<'a, M>,
}

impl<'a, M, const N: usize> ManagedType<'a, M> for ManagedByteArray<'a, M, N>
where
    M: ManagedTypeApi<'a>,
{
    type OwnHandle = M::ManagedBufferHandle;

    #[inline]
    fn from_handle(handle: M::ManagedBufferHandle) -> Self {
        ManagedByteArray {
            buffer: ManagedBuffer::from_handle(handle),
        }
    }

    unsafe fn get_handle(&self) -> M::ManagedBufferHandle {
        self.buffer.get_handle()
    }

    fn take_handle(self) -> Self::OwnHandle {
        self.buffer.take_handle()
    }

    fn transmute_from_handle_ref(handle_ref: &M::ManagedBufferHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<'a, M, const N: usize> Default for ManagedByteArray<'a, M, N>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    fn default() -> Self {
        Self::new_from_bytes(&[0u8; N])
    }
}

impl<'a, M, const N: usize> From<&[u8; N]> for ManagedByteArray<'a, M, N>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    fn from(bytes: &[u8; N]) -> Self {
        Self::new_from_bytes(bytes)
    }
}

impl<'a, M, const N: usize> ManagedByteArray<'a, M, N>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    pub fn new_from_bytes(bytes: &[u8; N]) -> Self {
        ManagedByteArray {
            buffer: ManagedBuffer::new_from_bytes(&bytes[..]),
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
    pub fn as_managed_buffer(&self) -> &ManagedBuffer<'a, M> {
        &self.buffer
    }

    #[inline]
    pub fn to_byte_array(&self) -> [u8; N] {
        let mut result = [0u8; N];
        let _ = self.buffer.load_slice(0, &mut result[..]);
        result
    }
}

impl<'a, M, const N: usize> PartialEq for ManagedByteArray<'a, M, N>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }
}

impl<'a, M, const N: usize> Eq for ManagedByteArray<'a, M, N> where M: ManagedTypeApi<'a> {}

impl<'a, M, const N: usize> TryFrom<ManagedBuffer<'a, M>> for ManagedByteArray<'a, M, N>
where
    M: ManagedTypeApi<'a>,
{
    type Error = DecodeError;

    fn try_from(value: ManagedBuffer<'a, M>) -> Result<Self, Self::Error> {
        if value.len() != N {
            return Err(DecodeError::from(DECODE_ERROR_BAD_LENGTH));
        }
        Ok(ManagedByteArray { buffer: value })
    }
}

impl<'a, M, const N: usize> TopEncode for ManagedByteArray<'a, M, N>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.buffer.top_encode_or_handle_err(output, h)
    }
}

impl<'a, M, const N: usize> TopDecode for ManagedByteArray<'a, M, N>
where
    M: ManagedTypeApi<'a>,
{
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        let buffer = ManagedBuffer::top_decode_or_handle_err(input, h)?;
        if buffer.len() != N {
            return Err(h.handle_error(DecodeError::from(DECODE_ERROR_BAD_LENGTH)));
        }
        Ok(ManagedByteArray { buffer })
    }
}

#[derive(Clone)]
pub(crate) struct ManagedBufferSizeContext(pub usize);

impl TryStaticCast for ManagedBufferSizeContext {}

impl<'a, M, const N: usize> NestedEncode for ManagedByteArray<'a, M, N>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        if O::supports_specialized_type::<ManagedBuffer<'a, M>>() {
            dest.push_specialized((), &self.buffer, h)
        } else {
            dest.write(self.buffer.to_boxed_bytes().as_slice());
            Ok(())
        }
    }
}

impl<'a, M, const N: usize> NestedDecode for ManagedByteArray<'a, M, N>
where
    M: ManagedTypeApi<'a>,
{
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        let buffer = if I::supports_specialized_type::<ManagedBuffer<'a, M>>() {
            input.read_specialized(ManagedBufferSizeContext(N), h)?
        } else {
            let byte_array = <[u8; N]>::dep_decode_or_handle_err(input, h)?;
            byte_array.as_ref().into()
        };
        Ok(ManagedByteArray { buffer })
    }
}

impl<'a, M, const N: usize> TypeAbi for ManagedByteArray<'a, M, N>
where
    M: ManagedTypeApi<'a>,
{
    /// It is semantically equivalent to `[u8; N]`.
    fn type_name() -> TypeName {
        <&[u8; N] as TypeAbi>::type_name()
    }
}

impl<'a, M, const N: usize> SCLowerHex<'a> for ManagedByteArray<'a, M, N>
where
    M: ManagedTypeApi<'a>,
{
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F) {
        SCLowerHex::fmt(&self.buffer, f)
    }
}

impl<'a, M, const N: usize> core::fmt::Debug for ManagedByteArray<'a, M, N>
where
    M: ManagedTypeApi<'a>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ManagedByteArray")
            .field("handle", &self.buffer.handle)
            .field("size", &N)
            .field("hex-value", &encode_bytes_as_hex(&self.to_byte_array()))
            .finish()
    }
}
