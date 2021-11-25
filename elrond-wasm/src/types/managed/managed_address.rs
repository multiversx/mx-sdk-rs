use core::convert::{TryFrom, TryInto};

use super::{ManagedBuffer, ManagedByteArray, ManagedType};
use crate::{
    abi::TypeAbi,
    api::{Handle, ManagedTypeApi},
    types::Address,
};
use alloc::string::String;
use elrond_codec::{
    DecodeError, EncodeError, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, TryStaticCast,
};

#[derive(Clone, Debug)]
pub struct ManagedAddress<M: ManagedTypeApi> {
    bytes: ManagedByteArray<M, 32>,
}

impl<M> ManagedAddress<M>
where
    M: ManagedTypeApi,
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
    pub fn as_managed_buffer(&self) -> &ManagedBuffer<M> {
        self.bytes.as_managed_buffer()
    }
}

impl<M> From<&Address> for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from(address: &Address) -> Self {
        Self::from_address(address)
    }
}

impl<M> From<Address> for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from(address: Address) -> Self {
        Self::from(&address)
    }
}

impl<M> From<&[u8; 32]> for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from(bytes: &[u8; 32]) -> Self {
        Self::new_from_bytes(bytes)
    }
}

impl<M> From<ManagedByteArray<M, 32>> for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    fn from(value: ManagedByteArray<M, 32>) -> Self {
        Self { bytes: value }
    }
}

impl<M> TryFrom<ManagedBuffer<M>> for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    type Error = DecodeError;

    fn try_from(value: ManagedBuffer<M>) -> Result<Self, Self::Error> {
        let bytes: ManagedByteArray<M, 32> = value.try_into()?;
        Ok(bytes.into())
    }
}

impl<M> ManagedType<M> for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from_raw_handle(handle: Handle) -> Self {
        ManagedAddress {
            bytes: ManagedByteArray::from_raw_handle(handle),
        }
    }

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle {
        self.bytes.get_raw_handle()
    }
}

impl<M> Default for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl<M> PartialEq for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl<M> Eq for ManagedAddress<M> where M: ManagedTypeApi {}

impl<M> TopEncode for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.bytes.top_encode(output)
    }
}

impl<M> TopDecode for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        Ok(ManagedAddress {
            bytes: ManagedByteArray::top_decode(input)?,
        })
    }
}

#[derive(Clone)]
pub(crate) struct ManagedBufferSizeContext(pub usize);

impl TryStaticCast for ManagedBufferSizeContext {}

impl<M> NestedEncode for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.bytes.dep_encode(dest)
    }
}

impl<M> NestedDecode for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(ManagedAddress {
            bytes: ManagedByteArray::dep_decode(input)?,
        })
    }
}

impl<M> TypeAbi for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    /// `"Address"` instead of `"array32<u8>"`.
    fn type_name() -> String {
        Address::type_name()
    }
}
