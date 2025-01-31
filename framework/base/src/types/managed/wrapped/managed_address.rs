use core::convert::{TryFrom, TryInto};

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{use_raw_handle, ManagedTypeApi, RawHandle},
    codec::{
        DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
        NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
    },
    formatter::{hex_util::encode_bytes_as_hex, FormatByteReceiver, SCLowerHex},
    types::{heap::Address, ManagedBuffer, ManagedByteArray, ManagedType},
};

use super::ManagedRef;

#[repr(transparent)]
#[derive(Clone)]
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

    /// Creates a new object, without initializing it.
    ///
    /// ## Safety
    ///
    /// The value needs to be initialized after creation, otherwise the VM will halt the first time the value is attempted to be read.
    pub unsafe fn new_uninit() -> Self {
        ManagedAddress {
            bytes: ManagedByteArray::new_uninit(),
        }
    }

    /// Creates a shared managed reference to a given raw handle.
    ///
    /// ## Safety
    ///
    /// The reference points to a shared value. Make sure the handle is not leaked.
    pub unsafe fn temp_const_ref(
        raw_handle: RawHandle,
    ) -> ManagedRef<'static, M, ManagedAddress<M>> {
        ManagedRef::wrap_handle(use_raw_handle(raw_handle))
    }

    #[inline]
    pub fn as_managed_buffer(&self) -> &ManagedBuffer<M> {
        self.bytes.as_managed_buffer()
    }

    #[inline]
    pub fn as_managed_byte_array(&self) -> &ManagedByteArray<M, 32> {
        &self.bytes
    }

    #[inline]
    pub fn to_byte_array(&self) -> [u8; 32] {
        self.bytes.to_byte_array()
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

impl<M> From<[u8; 32]> for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from(bytes: [u8; 32]) -> Self {
        Self::new_from_bytes(&bytes)
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
    type OwnHandle = M::ManagedBufferHandle;

    #[inline]
    unsafe fn from_handle(handle: M::ManagedBufferHandle) -> Self {
        ManagedAddress {
            bytes: ManagedByteArray::from_handle(handle),
        }
    }

    fn get_handle(&self) -> M::ManagedBufferHandle {
        self.bytes.get_handle()
    }

    unsafe fn forget_into_handle(self) -> Self::OwnHandle {
        self.bytes.forget_into_handle()
    }

    fn transmute_from_handle_ref(handle_ref: &M::ManagedBufferHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }

    fn transmute_from_handle_ref_mut(handle_ref: &mut M::ManagedBufferHandle) -> &mut Self {
        unsafe { core::mem::transmute(handle_ref) }
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
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.bytes.top_encode_or_handle_err(output, h)
    }
}

impl<M> TopDecode for ManagedAddress<M>
where
    M: ManagedTypeApi,
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

impl<M> NestedEncode for ManagedAddress<M>
where
    M: ManagedTypeApi,
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

impl<M> NestedDecode for ManagedAddress<M>
where
    M: ManagedTypeApi,
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

impl<M> TypeAbiFrom<Self> for ManagedAddress<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&Self> for ManagedAddress<M> where M: ManagedTypeApi {}

impl<M> TypeAbi for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    type Unmanaged = crate::types::heap::Address;

    /// `"Address"` instead of `"array32<u8>"`.
    fn type_name() -> TypeName {
        Address::type_name()
    }

    fn type_name_rust() -> TypeName {
        "ManagedAddress<$API>".into()
    }
}

impl<M: ManagedTypeApi> SCLowerHex for ManagedAddress<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        SCLowerHex::fmt(&self.bytes, f)
    }
}

impl<M: ManagedTypeApi> ManagedAddress<M> {
    pub fn hex_expr(&self) -> ManagedBuffer<M> {
        self.bytes.buffer.hex_expr()
    }
}

impl<M: ManagedTypeApi> core::fmt::Debug for ManagedAddress<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ManagedAddress")
            .field("handle", &self.bytes.buffer.handle)
            .field("hex-value", &encode_bytes_as_hex(&self.to_byte_array()))
            .finish()
    }
}

impl<M> TypeAbiFrom<[u8; 32]> for ManagedAddress<M> where M: ManagedTypeApi {}

impl<M> TypeAbiFrom<Address> for ManagedAddress<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&Address> for ManagedAddress<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<ManagedAddress<M>> for Address where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&ManagedAddress<M>> for Address where M: ManagedTypeApi {}
