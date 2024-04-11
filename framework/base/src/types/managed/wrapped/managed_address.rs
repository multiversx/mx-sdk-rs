use core::convert::{TryFrom, TryInto};

use crate::{
    abi::{TypeAbi, TypeName},
    api::ManagedTypeApi,
    codec::{
        CodecFrom, CodecFromSelf, DecodeError, DecodeErrorHandler, EncodeErrorHandler,
        NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput, TopDecode,
        TopDecodeInput, TopEncode, TopEncodeOutput, TryStaticCast,
    },
    formatter::{hex_util::encode_bytes_as_hex, FormatByteReceiver, SCLowerHex},
    types::{heap::Address, ManagedBuffer, ManagedByteArray, ManagedType},
};

#[repr(transparent)]
#[derive(Clone)]
pub struct ManagedAddress<'a, M: ManagedTypeApi<'a>> {
    bytes: ManagedByteArray<'a, M, 32>,
}

impl<'a, M> ManagedAddress<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    pub fn from_address(address: &Address) -> Self {
        Self::new_from_bytes(address.as_array())
    }

    #[inline]
    pub fn zero() -> Self {
        Self::new_from_bytes(&[0u8; 32])
    }

    pub fn to_address(&self) -> Address {
        let mut result = Address::zero();
        let _ = self.bytes.buffer.load_slice(0, result.as_mut());
        result
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.bytes.buffer == &[0u8; 32]
    }

    #[inline]
    pub fn new_from_bytes(bytes: &[u8; 32]) -> Self {
        ManagedAddress {
            bytes: ManagedByteArray::new_from_bytes(bytes),
        }
    }

    #[inline]
    pub fn as_managed_buffer(&self) -> &ManagedBuffer<'a, M> {
        self.bytes.as_managed_buffer()
    }

    #[inline]
    pub fn as_managed_byte_array(&self) -> &ManagedByteArray<'a, M, 32> {
        &self.bytes
    }

    #[inline]
    pub fn to_byte_array(&self) -> [u8; 32] {
        self.bytes.to_byte_array()
    }
}

impl<'a, M> From<&Address> for ManagedAddress<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    fn from(address: &Address) -> Self {
        Self::from_address(address)
    }
}

impl<'a, M> From<Address> for ManagedAddress<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    fn from(address: Address) -> Self {
        Self::from(&address)
    }
}

impl<'a, M> From<&[u8; 32]> for ManagedAddress<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    fn from(bytes: &[u8; 32]) -> Self {
        Self::new_from_bytes(bytes)
    }
}

impl<'a, M> From<[u8; 32]> for ManagedAddress<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    fn from(bytes: [u8; 32]) -> Self {
        Self::new_from_bytes(&bytes)
    }
}

impl<'a, M> From<ManagedByteArray<'a, M, 32>> for ManagedAddress<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    fn from(value: ManagedByteArray<'a, M, 32>) -> Self {
        Self { bytes: value }
    }
}

impl<'a, M> TryFrom<ManagedBuffer<'a, M>> for ManagedAddress<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    type Error = DecodeError;

    fn try_from(value: ManagedBuffer<'a, M>) -> Result<Self, Self::Error> {
        let bytes: ManagedByteArray<'a, M, 32> = value.try_into()?;
        Ok(bytes.into())
    }
}

impl<'a, M> ManagedType<'a, M> for ManagedAddress<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    type OwnHandle = M::ManagedBufferHandle;

    #[inline]
    fn from_handle(handle: M::ManagedBufferHandle) -> Self {
        ManagedAddress {
            bytes: ManagedByteArray::from_handle(handle),
        }
    }

    unsafe fn get_handle(&self) -> M::ManagedBufferHandle {
        self.bytes.get_handle()
    }

    fn take_handle(self) -> Self::OwnHandle {
        self.bytes.take_handle()
    }

    fn transmute_from_handle_ref(handle_ref: &M::ManagedBufferHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<'a, M> Default for ManagedAddress<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl<'a, M> PartialEq for ManagedAddress<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl<'a, M> Eq for ManagedAddress<'a, M> where M: ManagedTypeApi<'a> {}

impl<'a, M> TopEncode for ManagedAddress<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.bytes.top_encode_or_handle_err(output, h)
    }
}

impl<'a, M> TopDecode for ManagedAddress<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(ManagedAddress {
            bytes: ManagedByteArray::top_decode_or_handle_err(input, h)?,
        })
    }
}

#[derive(Clone)]
pub(crate) struct ManagedBufferSizeContext(pub usize);

impl TryStaticCast for ManagedBufferSizeContext {}

impl<'a, M> NestedEncode for ManagedAddress<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.bytes.dep_encode_or_handle_err(dest, h)
    }
}

impl<'a, M> NestedDecode for ManagedAddress<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(ManagedAddress {
            bytes: ManagedByteArray::dep_decode_or_handle_err(input, h)?,
        })
    }
}

impl<'a, M> TypeAbi for ManagedAddress<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    /// `"Address"` instead of `"array32<u8>"`.
    fn type_name() -> TypeName {
        Address::type_name()
    }
}

impl<'a, M: ManagedTypeApi<'a>> SCLowerHex<'a> for ManagedAddress<'a, M> {
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F) {
        SCLowerHex::fmt(&self.bytes, f)
    }
}

impl<'a, M: ManagedTypeApi<'a>> core::fmt::Debug for ManagedAddress<'a, M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ManagedAddress")
            .field("handle", &self.bytes.buffer.handle)
            .field("hex-value", &encode_bytes_as_hex(&self.to_byte_array()))
            .finish()
    }
}

impl<'a, M> CodecFromSelf for ManagedAddress<'a, M> where M: ManagedTypeApi<'a> {}

impl<'a, M> CodecFrom<[u8; 32]> for ManagedAddress<'a, M> where M: ManagedTypeApi<'a> {}

#[cfg(feature = "alloc")]
impl<'a, M> CodecFrom<Address> for ManagedAddress<'a, M> where M: ManagedTypeApi<'a> {}

#[cfg(feature = "alloc")]
impl<'a, M> CodecFrom<&Address> for ManagedAddress<'a, M> where M: ManagedTypeApi<'a> {}

#[cfg(feature = "alloc")]
impl<'a, M> CodecFrom<ManagedAddress<'a, M>> for Address where M: ManagedTypeApi<'a> {}

#[cfg(feature = "alloc")]
impl<'a, M> CodecFrom<&ManagedAddress<'a, M>> for Address where M: ManagedTypeApi<'a> {}
