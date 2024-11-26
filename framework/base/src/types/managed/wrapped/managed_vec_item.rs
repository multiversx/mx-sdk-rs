use core::borrow::Borrow;

use multiversx_chain_core::types::{EsdtLocalRole, EsdtTokenType};

use crate::{
    api::{use_raw_handle, ManagedTypeApi},
    types::{
        BigInt, BigUint, EllipticCurve, ManagedAddress, ManagedBuffer, ManagedByteArray,
        ManagedRef, ManagedType, ManagedVec, TokenIdentifier,
    },
};

use super::{ManagedVecItemPayload, ManagedVecItemPayloadAdd, ManagedVecItemPayloadBuffer};

/// Types that implement this trait can be items inside a `ManagedVec`.
/// All these types need a payload, i.e a representation that gets stored
/// in the underlying managed buffer.
/// Not all data needs to be stored as payload, for instance for most managed types
/// the payload is just the handle, whereas the mai ndata is kept by the VM.
pub trait ManagedVecItem: 'static {
    /// Type managing the underlying binary representation in a ManagedVec..
    type PAYLOAD: ManagedVecItemPayload;

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
    ///
    /// TODO: wrap other types in readonly wrapper.
    type Ref<'a>: Borrow<Self>;

    fn payload_size() -> usize {
        Self::PAYLOAD::payload_size()
    }

    /// Parses given bytes as a an owned object.
    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self;

    /// Parses given bytes as a representation of the object, either owned, or a reference.
    ///
    /// # Safety
    ///
    /// In certain cases this involves practically disregarding the lifetimes, hence it is unsafe.
    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a>;

    /// Converts the object into bytes.
    ///
    /// The output is processed by the `writer` lambda.
    /// The writer is provided by the caller.
    /// The callee will use it to pass on the bytes.
    ///
    /// The method is used when instering (push, overwrite) into a ManagedVec.
    ///
    /// Note that a destructor should not be called at this moment, since the ManagedVec will take ownership of the item.
    fn into_byte_writer<R, Writer: FnMut(&[u8]) -> R>(self, writer: Writer) -> R;
}

pub unsafe fn managed_vec_item_read_from_payload_index<T, P>(payload: &P, index: &mut usize) -> T
where
    T: ManagedVecItem,
    P: ManagedVecItemPayload,
{
    let value = T::read_from_payload(payload.slice_unchecked(*index));
    *index += T::PAYLOAD::payload_size();
    value
}

macro_rules! impl_int {
    ($ty:ident, $payload_size:expr) => {
        impl ManagedVecItem for $ty {
            type PAYLOAD = ManagedVecItemPayloadBuffer<$payload_size>;
            const SKIPS_RESERIALIZATION: bool = true;
            type Ref<'a> = Self;

            fn from_byte_reader<Reader: FnMut(&mut [u8])>(mut reader: Reader) -> Self {
                let mut arr: [u8; $payload_size] = [0u8; $payload_size];
                reader(&mut arr[..]);
                $ty::from_be_bytes(arr)
            }

            fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
                $ty::from_be_bytes(payload.buffer)
            }

            unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
                reader: Reader,
            ) -> Self::Ref<'a> {
                Self::from_byte_reader(reader)
            }

            fn into_byte_writer<R, Writer: FnMut(&[u8]) -> R>(self, mut writer: Writer) -> R {
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
    type PAYLOAD = ManagedVecItemPayloadBuffer<4>;
    const SKIPS_RESERIALIZATION: bool = true;
    type Ref<'a> = Self;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(mut reader: Reader) -> Self {
        let mut arr: [u8; 4] = [0u8; 4];
        reader(&mut arr[..]);
        u32::from_be_bytes(arr) as usize
    }

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        u32::read_from_payload(payload) as usize
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }

    fn into_byte_writer<R, Writer: FnMut(&[u8]) -> R>(self, mut writer: Writer) -> R {
        let bytes = (self as u32).to_be_bytes();
        writer(&bytes)
    }
}

impl ManagedVecItem for bool {
    type PAYLOAD = ManagedVecItemPayloadBuffer<1>;
    const SKIPS_RESERIALIZATION: bool = true;
    type Ref<'a> = Self;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
        u8::from_byte_reader(reader) > 0
    }

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        u8::read_from_payload(payload) > 0
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }

    fn into_byte_writer<R, Writer: FnMut(&[u8]) -> R>(self, writer: Writer) -> R {
        // true -> 1u8
        // false -> 0u8
        let u8_value = u8::from(self);
        <u8 as ManagedVecItem>::into_byte_writer(u8_value, writer)
    }
}

impl<T> ManagedVecItem for Option<T>
where
    ManagedVecItemPayloadBuffer<1>: ManagedVecItemPayloadAdd<T::PAYLOAD>,
    T: ManagedVecItem,
{
    type PAYLOAD = <ManagedVecItemPayloadBuffer<1> as ManagedVecItemPayloadAdd<T::PAYLOAD>>::Output;
    const SKIPS_RESERIALIZATION: bool = false;
    type Ref<'a> = Self;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(mut reader: Reader) -> Self {
        let mut payload = Self::PAYLOAD::new_buffer();
        let payload_slice = payload.payload_slice_mut();
        reader(payload_slice);
        if payload_slice[0] == 0 {
            None
        } else {
            Some(T::from_byte_reader(|bytes| {
                bytes.copy_from_slice(&payload_slice[1..]);
            }))
        }
    }

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let (p1, p2) = <ManagedVecItemPayloadBuffer<1> as ManagedVecItemPayloadAdd<
            T::PAYLOAD,
        >>::split_from_add(payload);

        let disc = u8::read_from_payload(p1);
        if disc == 0 {
            None
        } else {
            Some(T::read_from_payload(p2))
        }
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }

    fn into_byte_writer<R, Writer: FnMut(&[u8]) -> R>(self, mut writer: Writer) -> R {
        let mut payload = Self::PAYLOAD::new_buffer();
        let slice = payload.payload_slice_mut();
        if let Some(t) = self {
            slice[0] = 1;
            T::into_byte_writer(t, |bytes| {
                slice[1..].copy_from_slice(bytes);
            });
        }
        writer(slice)
    }
}

macro_rules! impl_managed_type {
    ($ty:ident) => {
        impl<M: ManagedTypeApi> ManagedVecItem for $ty<M> {
            type PAYLOAD = ManagedVecItemPayloadBuffer<4>;
            const SKIPS_RESERIALIZATION: bool = false;
            type Ref<'a> = ManagedRef<'a, M, Self>;

            fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
                let handle = <$ty<M> as ManagedType<M>>::OwnHandle::from_byte_reader(reader);
                unsafe { $ty::from_handle(handle) }
            }

            fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
                let handle = use_raw_handle(i32::read_from_payload(payload));
                unsafe { Self::from_handle(handle) }
            }

            unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
                reader: Reader,
            ) -> Self::Ref<'a> {
                let handle = <$ty<M> as ManagedType<M>>::OwnHandle::from_byte_reader(reader);
                ManagedRef::wrap_handle(handle)
            }

            fn into_byte_writer<R, Writer: FnMut(&[u8]) -> R>(self, writer: Writer) -> R {
                let handle = unsafe { self.forget_into_handle() };
                <$ty<M> as ManagedType<M>>::OwnHandle::into_byte_writer(handle, writer)
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
    type PAYLOAD = ManagedVecItemPayloadBuffer<4>;
    const SKIPS_RESERIALIZATION: bool = false;
    type Ref<'a> = ManagedRef<'a, M, Self>;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
        let handle = <Self as ManagedType<M>>::OwnHandle::from_byte_reader(reader);
        unsafe { Self::from_handle(handle) }
    }

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let handle = use_raw_handle(i32::read_from_payload(payload));
        unsafe { Self::from_handle(handle) }
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        let handle = <Self as ManagedType<M>>::OwnHandle::from_byte_reader(reader);
        ManagedRef::wrap_handle(handle)
    }

    fn into_byte_writer<R, Writer: FnMut(&[u8]) -> R>(self, writer: Writer) -> R {
        <<Self as ManagedType<M>>::OwnHandle as ManagedVecItem>::into_byte_writer(
            self.get_handle(),
            writer,
        )
    }
}

impl<M, T> ManagedVecItem for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    type PAYLOAD = ManagedVecItemPayloadBuffer<4>;
    const SKIPS_RESERIALIZATION: bool = false;
    type Ref<'a> = ManagedRef<'a, M, Self>;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
        let handle = M::ManagedBufferHandle::from_byte_reader(reader);
        unsafe { Self::from_handle(handle) }
    }

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let handle = use_raw_handle(i32::read_from_payload(payload));
        unsafe { Self::from_handle(handle) }
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        let handle = M::ManagedBufferHandle::from_byte_reader(reader);
        ManagedRef::wrap_handle(handle)
    }

    fn into_byte_writer<R, Writer: FnMut(&[u8]) -> R>(self, writer: Writer) -> R {
        let handle = unsafe { self.forget_into_handle() };
        <M::ManagedBufferHandle as ManagedVecItem>::into_byte_writer(handle, writer)
    }
}

impl ManagedVecItem for EsdtTokenType {
    type PAYLOAD = ManagedVecItemPayloadBuffer<1>;
    const SKIPS_RESERIALIZATION: bool = true;
    type Ref<'a> = Self;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(mut reader: Reader) -> Self {
        let mut arr: [u8; 1] = [0u8; 1];
        reader(&mut arr[..]);
        arr[0].into()
    }

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        u8::read_from_payload(payload).into()
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }

    fn into_byte_writer<R, Writer: FnMut(&[u8]) -> R>(self, mut writer: Writer) -> R {
        writer(&[self.as_u8()])
    }
}

impl ManagedVecItem for EsdtLocalRole {
    type PAYLOAD = ManagedVecItemPayloadBuffer<2>;
    const SKIPS_RESERIALIZATION: bool = false; // TODO: might be ok to be true, but needs testing
    type Ref<'a> = Self;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
        u16::from_byte_reader(reader).into()
    }

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        u16::read_from_payload(payload).into()
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }

    fn into_byte_writer<R, Writer: FnMut(&[u8]) -> R>(self, writer: Writer) -> R {
        <u16 as ManagedVecItem>::into_byte_writer(self.as_u16(), writer)
    }
}
