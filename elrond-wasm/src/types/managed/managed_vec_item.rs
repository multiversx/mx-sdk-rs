use core::borrow::Borrow;

use crate::{
    api::{Handle, ManagedTypeApi},
    types::TokenIdentifier,
};

use super::{
    BigInt, BigUint, EllipticCurve, ManagedAddress, ManagedBuffer, ManagedByteArray, ManagedRef,
    ManagedType, ManagedVec,
};

/// Types that implement this trait can be items inside a `ManagedVec`.
/// All these types need a payload, i.e a representation that gets stored
/// in the underlying managed buffer.
/// Not all data needs to be stored as payload, for instance for most managed types
/// the payload is just the handle, whereas the mai ndata is kept by the VM.
pub trait ManagedVecItem: 'static {
    /// Size of the data stored in the underlying `ManagedBuffer`.
    const PAYLOAD_SIZE: usize;

    /// If true, then the encoding of the item is identical to the payload,
    /// and no further conversion is necessary
    /// (the underlying buffer can be used as-is during serialization).
    /// False for all managed types, but true for basic types (like `u32`).
    const SKIPS_RESERIALIZATION: bool;

    /// Reference representation of the ManagedVec item.
    ///
    /// Implementations:
    /// - For items with Copy semantics, it should be the type itself.
    /// - For managed types, ManagedRef does the job.
    /// - For any other types, `Self` is currently used, although this is technically unsafe.
    /// TODO: wrap other types in readonly wrapper.
    type Ref<'a>: Borrow<Self>;

    /// Parses given bytes as a an owned object.
    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self;

    /// Parses given bytes as a representation of the object, either owned, or a reference.
    ///
    /// # Safety
    ///
    /// In certain cases this involves practically disregarding the lifetimes, hence it is unsafe.
    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a>;

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R;
}

macro_rules! impl_int {
    ($ty:ident, $payload_size:expr) => {
        impl ManagedVecItem for $ty {
            const PAYLOAD_SIZE: usize = $payload_size;
            const SKIPS_RESERIALIZATION: bool = true;
            type Ref<'a> = Self;
            fn from_byte_reader<Reader: FnMut(&mut [u8])>(mut reader: Reader) -> Self {
                let mut arr: [u8; $payload_size] = [0u8; $payload_size];
                reader(&mut arr[..]);
                $ty::from_be_bytes(arr)
            }
            unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
                reader: Reader,
            ) -> Self::Ref<'a> {
                Self::from_byte_reader(reader)
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

impl ManagedVecItem for usize {
    const PAYLOAD_SIZE: usize = 4;
    const SKIPS_RESERIALIZATION: bool = true;
    type Ref<'a> = Self;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(mut reader: Reader) -> Self {
        let mut arr: [u8; 4] = [0u8; 4];
        reader(&mut arr[..]);
        u32::from_be_bytes(arr) as usize
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, mut writer: Writer) -> R {
        let bytes = (*self as u32).to_be_bytes();
        writer(&bytes)
    }
}

impl ManagedVecItem for bool {
    const PAYLOAD_SIZE: usize = 1;
    const SKIPS_RESERIALIZATION: bool = true;
    type Ref<'a> = Self;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
        u8::from_byte_reader(reader) > 0
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
        let u8_value = if *self { 1u8 } else { 0u8 };
        <u8 as ManagedVecItem>::to_byte_writer(&u8_value, writer)
    }
}

macro_rules! impl_managed_type {
    ($ty:ident) => {
        impl<M: ManagedTypeApi> ManagedVecItem for $ty<M> {
            const PAYLOAD_SIZE: usize = 4;
            const SKIPS_RESERIALIZATION: bool = false;
            type Ref<'a> = ManagedRef<'a, M, Self>;

            fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
                let handle = Handle::from_byte_reader(reader);
                $ty::from_raw_handle(handle)
            }

            unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
                reader: Reader,
            ) -> Self::Ref<'a> {
                let handle = Handle::from_byte_reader(reader);
                ManagedRef::wrap_handle(handle)
            }

            fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
                <Handle as ManagedVecItem>::to_byte_writer(&self.get_raw_handle(), writer)
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

impl<M, const N: usize> ManagedVecItem for ManagedByteArray<M, N>
where
    M: ManagedTypeApi,
{
    const PAYLOAD_SIZE: usize = 4;
    const SKIPS_RESERIALIZATION: bool = false;
    type Ref<'a> = ManagedRef<'a, M, Self>;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
        let handle = Handle::from_byte_reader(reader);
        Self::from_raw_handle(handle)
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        let handle = Handle::from_byte_reader(reader);
        ManagedRef::wrap_handle(handle)
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
        <Handle as ManagedVecItem>::to_byte_writer(&self.get_raw_handle(), writer)
    }
}

impl<M, T> ManagedVecItem for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    const PAYLOAD_SIZE: usize = 4;
    const SKIPS_RESERIALIZATION: bool = false;
    type Ref<'a> = ManagedRef<'a, M, Self>;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
        let handle = Handle::from_byte_reader(reader);
        Self::from_raw_handle(handle)
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        let handle = Handle::from_byte_reader(reader);
        ManagedRef::wrap_handle(handle)
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
        <Handle as ManagedVecItem>::to_byte_writer(&self.get_raw_handle(), writer)
    }
}
