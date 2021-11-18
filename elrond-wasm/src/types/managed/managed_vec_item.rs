use crate::{
    api::{Handle, ManagedTypeApi},
    types::TokenIdentifier,
};

use super::{
    BigInt, BigUint, EllipticCurve, ManagedAddress, ManagedBuffer, ManagedByteArray, ManagedType,
    ManagedVec,
};

/// Types that implement this trait can be items inside a `ManagedVec`.
/// All these types need a payload, i.e a representation that gets stored
/// in the underlying managed buffer.
/// Not all data needs to be stored as payload, for instance for most managed types
/// the payload is just the handle, whereas the mai ndata is kept by the VM.
pub trait ManagedVecItem<M: ManagedTypeApi> {
    /// Size of the data stored in the underlying `ManagedBuffer`.
    const PAYLOAD_SIZE: usize;

    /// If true, then the encoding of the item is identical to the payload,
    /// and no further conversion is necessary
    /// (the underlying buffer can be used as-is during serialization).
    /// False for all managed types, but true for basic types (like `u32`).
    const SKIPS_RESERIALIZATION: bool;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(api: M, reader: Reader) -> Self;

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R;
}

macro_rules! impl_int {
    ($ty:ident, $payload_size:expr) => {
        impl<M: ManagedTypeApi> ManagedVecItem<M> for $ty {
            const PAYLOAD_SIZE: usize = $payload_size;
            const SKIPS_RESERIALIZATION: bool = true;

            fn from_byte_reader<Reader: FnMut(&mut [u8])>(_api: M, mut reader: Reader) -> Self {
                let mut arr: [u8; $payload_size] = [0u8; $payload_size];
                reader(&mut arr[..]);
                $ty::from_be_bytes(arr)
            }

            fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, mut writer: Writer) -> R {
                let bytes = self.to_be_bytes();
                writer(&bytes)
            }
        }
    };
}
impl_int! {u8, 1}
impl_int! {u16, 2}
impl_int! {u32, 4}
impl_int! {u64, 8}
impl_int! {i32, 4}
impl_int! {i64, 8}

impl<M: ManagedTypeApi> ManagedVecItem<M> for usize {
    const PAYLOAD_SIZE: usize = 4;
    const SKIPS_RESERIALIZATION: bool = true;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(_api: M, mut reader: Reader) -> Self {
        let mut arr: [u8; 4] = [0u8; 4];
        reader(&mut arr[..]);
        u32::from_be_bytes(arr) as usize
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, mut writer: Writer) -> R {
        let bytes = (*self as u32).to_be_bytes();
        writer(&bytes)
    }
}

impl<M: ManagedTypeApi> ManagedVecItem<M> for bool {
    const PAYLOAD_SIZE: usize = 1;
    const SKIPS_RESERIALIZATION: bool = true;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(api: M, reader: Reader) -> Self {
        u8::from_byte_reader(api, reader) > 0
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
        let u8_value = if *self { 1u8 } else { 0u8 };
        <u8 as ManagedVecItem<M>>::to_byte_writer(&u8_value, writer)
    }
}

macro_rules! impl_managed_type {
    ($ty:ident) => {
        impl<M: ManagedTypeApi> ManagedVecItem<M> for $ty<M> {
            const PAYLOAD_SIZE: usize = 4;
            const SKIPS_RESERIALIZATION: bool = false;

            fn from_byte_reader<Reader: FnMut(&mut [u8])>(api: M, reader: Reader) -> Self {
                let handle = Handle::from_byte_reader(api.clone(), reader);
                $ty::from_raw_handle(api, handle)
            }

            fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
                <Handle as ManagedVecItem<M>>::to_byte_writer(&self.get_raw_handle(), writer)
            }
        }
    };
}

impl_managed_type! {ManagedBuffer}
impl_managed_type! {BigUint}
impl_managed_type! {BigInt}
impl_managed_type! {EllipticCurve}
impl_managed_type! {ManagedAddress}
impl_managed_type! {TokenIdentifier}

impl<M, const N: usize> ManagedVecItem<M> for ManagedByteArray<M, N>
where
    M: ManagedTypeApi,
{
    const PAYLOAD_SIZE: usize = 4;
    const SKIPS_RESERIALIZATION: bool = false;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(api: M, reader: Reader) -> Self {
        let handle = Handle::from_byte_reader(api.clone(), reader);
        Self::from_raw_handle(api, handle)
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
        <Handle as ManagedVecItem<M>>::to_byte_writer(&self.get_raw_handle(), writer)
    }
}

impl<M, T> ManagedVecItem<M> for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    const PAYLOAD_SIZE: usize = 4;
    const SKIPS_RESERIALIZATION: bool = false;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(api: M, reader: Reader) -> Self {
        let handle = Handle::from_byte_reader(api.clone(), reader);
        Self::from_raw_handle(api, handle)
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
        <Handle as ManagedVecItem<M>>::to_byte_writer(&self.get_raw_handle(), writer)
    }
}
