use super::{ManagedByteArray, ManagedDefault, ManagedFrom, ManagedType};
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
    pub fn from_address(api: M, address: &Address) -> Self {
        Self::new_from_bytes(api, address.as_array())
    }

    #[inline]
    pub fn zero(api: M) -> Self {
        Self::new_from_bytes(api, &[0u8; 32])
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
    pub fn new_from_bytes(api: M, bytes: &[u8; 32]) -> Self {
        ManagedAddress {
            bytes: ManagedByteArray::new_from_bytes(api, bytes),
        }
    }
}

impl<M> ManagedFrom<M, &Address> for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn managed_from(api: M, address: &Address) -> Self {
        Self::from_address(api, address)
    }
}

impl<M> ManagedFrom<M, Address> for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn managed_from(api: M, address: Address) -> Self {
        Self::managed_from(api, &address)
    }
}

impl<M> ManagedType<M> for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from_raw_handle(api: M, handle: Handle) -> Self {
        ManagedAddress {
            bytes: ManagedByteArray::from_raw_handle(api, handle),
        }
    }

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle {
        self.bytes.get_raw_handle()
    }

    #[inline]
    fn type_manager(&self) -> M {
        self.bytes.type_manager()
    }
}

impl<M> ManagedDefault<M> for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn managed_default(api: M) -> Self {
        Self::zero(api)
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
