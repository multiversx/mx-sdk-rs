use core::borrow::Borrow;

use multiversx_chain_core::types::{EsdtLocalRole, EsdtTokenType};
use multiversx_sc_codec::multi_types::{MultiValue2, MultiValue3};

use crate::{
    api::{use_raw_handle, HandleConstraints, ManagedTypeApi},
    types::{
        BigInt, BigUint, EllipticCurve, ManagedAddress, ManagedBuffer, ManagedByteArray,
        ManagedRef, ManagedType, ManagedVec, TokenIdentifier,
    },
};

use super::{
    ManagedVecItemNestedTuple, ManagedVecItemPayload, ManagedVecItemPayloadAdd,
    ManagedVecItemPayloadBuffer,
};

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
    fn read_from_payload(payload: &Self::PAYLOAD) -> Self;

    /// Parses given bytes as a representation of the object, either owned, or a reference.
    ///
    /// # Safety
    ///
    /// In certain cases this involves practically disregarding the lifetimes, hence it is unsafe.
    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a>;

    /// Converts the object into bytes.
    ///
    /// The method is used when instering (push, overwrite) into a ManagedVec.
    ///
    /// Note that a destructor should not be called at this moment, since the ManagedVec will take ownership of the item.
    fn save_to_payload(self, payload: &mut Self::PAYLOAD);
}

/// Used by the ManagedVecItem derive.
///
/// ## Safety
///
/// Only works correctly if the given index is correct, otherwise undefined behavior is possible.
pub unsafe fn managed_vec_item_read_from_payload_index<T, P>(payload: &P, index: &mut usize) -> T
where
    T: ManagedVecItem,
    P: ManagedVecItemPayload,
{
    let value = T::read_from_payload(payload.slice_unchecked(*index));
    *index += T::PAYLOAD::payload_size();
    value
}

/// Used by the ManagedVecItem derive.
///
/// ## Safety
///
/// Only works correctly if the given index is correct, otherwise undefined behavior is possible.
pub unsafe fn managed_vec_item_save_to_payload_index<T, P>(
    item: T,
    payload: &mut P,
    index: &mut usize,
) where
    T: ManagedVecItem,
    P: ManagedVecItemPayload,
{
    item.save_to_payload(payload.slice_unchecked_mut(*index));
    *index += T::PAYLOAD::payload_size();
}

macro_rules! impl_int {
    ($ty:ident, $payload_size:expr) => {
        impl ManagedVecItem for $ty {
            type PAYLOAD = ManagedVecItemPayloadBuffer<$payload_size>;
            const SKIPS_RESERIALIZATION: bool = true;
            type Ref<'a> = Self;

            fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
                $ty::from_be_bytes(payload.buffer)
            }

            unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
                $ty::from_be_bytes(payload.buffer)
            }

            fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
                payload.buffer = self.to_be_bytes();
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

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        u32::read_from_payload(payload) as usize
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        Self::read_from_payload(payload)
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        (self as u32).save_to_payload(payload);
    }
}

impl ManagedVecItem for bool {
    type PAYLOAD = ManagedVecItemPayloadBuffer<1>;
    const SKIPS_RESERIALIZATION: bool = true;
    type Ref<'a> = Self;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        u8::read_from_payload(payload) > 0
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        Self::read_from_payload(payload)
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        // true -> 1u8
        // false -> 0u8
        u8::from(self).save_to_payload(payload);
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

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        Self::read_from_payload(payload)
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        let (p1, p2) = <ManagedVecItemPayloadBuffer<1> as ManagedVecItemPayloadAdd<
            T::PAYLOAD,
        >>::split_mut_from_add(payload);

        if let Some(t) = self {
            1u8.save_to_payload(p1);
            t.save_to_payload(p2);
        }
    }
}

macro_rules! impl_managed_type {
    ($ty:ident) => {
        impl<M: ManagedTypeApi> ManagedVecItem for $ty<M> {
            type PAYLOAD = ManagedVecItemPayloadBuffer<4>;
            const SKIPS_RESERIALIZATION: bool = false;
            type Ref<'a> = ManagedRef<'a, M, Self>;

            fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
                let handle = use_raw_handle(i32::read_from_payload(payload));
                unsafe { Self::from_handle(handle) }
            }

            unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
                let handle = use_raw_handle(i32::read_from_payload(payload));
                ManagedRef::wrap_handle(handle)
            }

            fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
                let handle = unsafe { self.forget_into_handle() };
                handle.get_raw_handle().save_to_payload(payload);
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

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let handle = use_raw_handle(i32::read_from_payload(payload));
        unsafe { Self::from_handle(handle) }
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        let handle = use_raw_handle(i32::read_from_payload(payload));
        ManagedRef::wrap_handle(handle)
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        let handle = unsafe { self.forget_into_handle() };
        handle.get_raw_handle().save_to_payload(payload);
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

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let handle = use_raw_handle(i32::read_from_payload(payload));
        unsafe { Self::from_handle(handle) }
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        let handle = use_raw_handle(i32::read_from_payload(payload));
        ManagedRef::wrap_handle(handle)
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        let handle = unsafe { self.forget_into_handle() };
        handle.get_raw_handle().save_to_payload(payload);
    }
}

impl ManagedVecItem for EsdtTokenType {
    type PAYLOAD = ManagedVecItemPayloadBuffer<1>;
    const SKIPS_RESERIALIZATION: bool = true;
    type Ref<'a> = Self;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        u8::read_from_payload(payload).into()
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        Self::read_from_payload(payload)
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        self.as_u8().save_to_payload(payload);
    }
}

impl ManagedVecItem for EsdtLocalRole {
    type PAYLOAD = ManagedVecItemPayloadBuffer<2>;
    const SKIPS_RESERIALIZATION: bool = false; // TODO: might be ok to be true, but needs testing
    type Ref<'a> = Self;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        u16::read_from_payload(payload).into()
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        Self::read_from_payload(payload)
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        self.as_u16().save_to_payload(payload);
    }
}

impl<T1, T2> ManagedVecItem for MultiValue2<T1, T2>
where
    T1: ManagedVecItem,
    T2: ManagedVecItem,
    (T1, (T2, ())): ManagedVecItemNestedTuple,
{
    type PAYLOAD = <(T1, (T2, ())) as ManagedVecItemNestedTuple>::PAYLOAD;
    const SKIPS_RESERIALIZATION: bool = T1::SKIPS_RESERIALIZATION && T2::SKIPS_RESERIALIZATION;
    type Ref<'a> = Self;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let mut index = 0;
        unsafe {
            (
                managed_vec_item_read_from_payload_index(payload, &mut index),
                managed_vec_item_read_from_payload_index(payload, &mut index),
            )
                .into()
        }
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        // TODO: tuple of refs
        Self::read_from_payload(payload)
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        let tuple = self.into_tuple();
        let mut index = 0;

        unsafe {
            managed_vec_item_save_to_payload_index(tuple.0, payload, &mut index);
            managed_vec_item_save_to_payload_index(tuple.1, payload, &mut index);
        }
    }
}

impl<T1, T2, T3> ManagedVecItem for MultiValue3<T1, T2, T3>
where
    T1: ManagedVecItem,
    T2: ManagedVecItem,
    T3: ManagedVecItem,
    (T1, (T2, (T3, ()))): ManagedVecItemNestedTuple,
{
    type PAYLOAD = <(T1, (T2, (T3, ()))) as ManagedVecItemNestedTuple>::PAYLOAD;
    const SKIPS_RESERIALIZATION: bool = T1::SKIPS_RESERIALIZATION && T2::SKIPS_RESERIALIZATION;
    type Ref<'a> = Self;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let mut index = 0;
        unsafe {
            (
                managed_vec_item_read_from_payload_index(payload, &mut index),
                managed_vec_item_read_from_payload_index(payload, &mut index),
                managed_vec_item_read_from_payload_index(payload, &mut index),
            )
                .into()
        }
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        // TODO: tuple of refs
        Self::read_from_payload(payload)
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        let tuple = self.into_tuple();
        let mut index = 0;

        unsafe {
            managed_vec_item_save_to_payload_index(tuple.0, payload, &mut index);
            managed_vec_item_save_to_payload_index(tuple.1, payload, &mut index);
            managed_vec_item_save_to_payload_index(tuple.2, payload, &mut index);
        }
    }
}
