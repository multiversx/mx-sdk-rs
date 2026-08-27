use alloc::{boxed::Box, string::String, vec::Vec};

use crate::{
    abi::TypeAbi,
    api::{ErrorApi, ManagedTypeApi},
    chain_core::types::{
        Address, BLSKey, BLSSignature, BoxedBytes, CodeMetadata, DurationMillis, DurationSeconds,
        EsdtLocalRole, EsdtTokenType, H256, TimestampMillis, TimestampSeconds,
    },
    codec::{arrayvec::ArrayVec, multi_types::*},
};

pub trait HasUnmanaged: TypeAbi {
    type Unmanaged;
}

macro_rules! unmanaged_self {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl HasUnmanaged for $ty {
                type Unmanaged = Self;
            }
        )+
    };
}

unmanaged_self!(
    (),
    u8,
    u16,
    u32,
    usize,
    u64,
    u128,
    i8,
    i16,
    i32,
    isize,
    i64,
    core::num::NonZeroUsize,
    bool,
    f64,
    String,
    &'static str,
    Box<str>,
    H256,
    BoxedBytes,
    CodeMetadata,
    BLSKey,
    BLSSignature,
    EsdtTokenType,
    EsdtLocalRole,
    Address,
    DurationMillis,
    DurationSeconds,
    TimestampMillis,
    TimestampSeconds,
    IgnoreValue,
    crate::abi::AddressAbi,
    crate::abi::BigFloatAbi,
    crate::abi::BigIntAbi,
    crate::abi::DecimalAbi,
    crate::abi::DecimalSignedAbi,
    crate::abi::EgldOrEsdtTokenIdentifierAbi,
    crate::abi::EllipticCurveAbi,
    crate::abi::EsdtTokenIdentifierAbi,
    crate::abi::FungiblePaymentAbi,
    crate::abi::NonZeroBigUintAbi,
    crate::abi::PaymentAbi,
    crate::abi::SignAbi,
    crate::abi::TokenIdAbi,
    crate::abi::BytesReadToEndAbi,
);

impl<const DECIMALS: usize> HasUnmanaged for crate::abi::DecimalConstAbi<DECIMALS> {
    type Unmanaged = Self;
}

impl<const DECIMALS: usize> HasUnmanaged for crate::abi::DecimalSignedConstAbi<DECIMALS> {
    type Unmanaged = Self;
}

impl<T: crate::abi::AbiType> HasUnmanaged for crate::abi::ListAbi<T> {
    type Unmanaged = Self;
}

impl<T: crate::abi::AbiType> HasUnmanaged for crate::abi::CountedVariadicAbi<T> {
    type Unmanaged = Self;
}

#[cfg(feature = "num-bigint")]
impl HasUnmanaged for crate::abi::BigUintAbi {
    type Unmanaged = crate::codec::num_bigint::BigUint;
}

#[cfg(not(feature = "num-bigint"))]
impl HasUnmanaged for crate::abi::BigUintAbi {
    type Unmanaged = Self;
}

#[cfg(feature = "num-bigint")]
unmanaged_self!(
    crate::codec::num_bigint::BigUint,
    crate::codec::num_bigint::BigInt
);

impl<T: HasUnmanaged> HasUnmanaged for &T {
    type Unmanaged = T::Unmanaged;
}

impl<T: HasUnmanaged> HasUnmanaged for Box<T> {
    type Unmanaged = Box<T::Unmanaged>;
}

impl<T: HasUnmanaged> HasUnmanaged for Vec<T> {
    type Unmanaged = Vec<T::Unmanaged>;
}

impl<T: TypeAbi, const CAP: usize> HasUnmanaged for ArrayVec<T, CAP> {
    type Unmanaged = Self;
}

impl<T: TypeAbi> HasUnmanaged for Box<[T]> {
    type Unmanaged = Self;
}

impl<T: HasUnmanaged> HasUnmanaged for Option<T> {
    type Unmanaged = Option<T::Unmanaged>;
}

impl<T: HasUnmanaged, E> HasUnmanaged for Result<T, E> {
    type Unmanaged = Result<T::Unmanaged, E>;
}

impl<T: HasUnmanaged, const N: usize> HasUnmanaged for [T; N] {
    type Unmanaged = [T::Unmanaged; N];
}

impl<T: HasUnmanaged> HasUnmanaged for MultiValueVec<T> {
    type Unmanaged = MultiValueVec<T::Unmanaged>;
}

impl<T: HasUnmanaged> HasUnmanaged for OptionalValue<T> {
    type Unmanaged = OptionalValue<T::Unmanaged>;
}

macro_rules! tuple_unmanaged {
    ($($type_name:ident),+) => {
        impl<$($type_name: HasUnmanaged),+> HasUnmanaged for ($($type_name,)+) {
            type Unmanaged = ($($type_name::Unmanaged,)+);
        }
    };
}

tuple_unmanaged!(T0);
tuple_unmanaged!(T0, T1);
tuple_unmanaged!(T0, T1, T2);
tuple_unmanaged!(T0, T1, T2, T3);
tuple_unmanaged!(T0, T1, T2, T3, T4);
tuple_unmanaged!(T0, T1, T2, T3, T4, T5);
tuple_unmanaged!(T0, T1, T2, T3, T4, T5, T6);
tuple_unmanaged!(T0, T1, T2, T3, T4, T5, T6, T7);
tuple_unmanaged!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
tuple_unmanaged!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
tuple_unmanaged!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
tuple_unmanaged!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
tuple_unmanaged!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
tuple_unmanaged!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
tuple_unmanaged!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14
);
tuple_unmanaged!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);

macro_rules! multi_value_unmanaged {
    ($type_name:ident: $($param:ident),+) => {
        impl<$($param: HasUnmanaged),+> HasUnmanaged for $type_name<$($param,)+> {
            type Unmanaged = $type_name<$($param::Unmanaged,)+>;
        }
    };
}

multi_value_unmanaged!(MultiValue2: T0, T1);
multi_value_unmanaged!(MultiValue3: T0, T1, T2);
multi_value_unmanaged!(MultiValue4: T0, T1, T2, T3);
multi_value_unmanaged!(MultiValue5: T0, T1, T2, T3, T4);
multi_value_unmanaged!(MultiValue6: T0, T1, T2, T3, T4, T5);
multi_value_unmanaged!(MultiValue7: T0, T1, T2, T3, T4, T5, T6);
multi_value_unmanaged!(MultiValue8: T0, T1, T2, T3, T4, T5, T6, T7);
multi_value_unmanaged!(MultiValue9: T0, T1, T2, T3, T4, T5, T6, T7, T8);
multi_value_unmanaged!(MultiValue10: T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
multi_value_unmanaged!(MultiValue11: T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
multi_value_unmanaged!(MultiValue12: T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
multi_value_unmanaged!(MultiValue13: T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
multi_value_unmanaged!(MultiValue14: T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
multi_value_unmanaged!(MultiValue15: T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
multi_value_unmanaged!(MultiValue16: T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);

unmanaged_self!(
    super::MessageHashType,
    super::heap::ArgBuffer,
    super::OperationCompletionStatus,
    super::Sign,
);

impl<T: TypeAbi> HasUnmanaged for super::heap::AsyncCallResult<T> {
    type Unmanaged = Self;
}

impl<T: TypeAbi> HasUnmanaged for super::heap::Queue<T> {
    type Unmanaged = Self;
}

impl<T: TypeAbi, E> HasUnmanaged for super::SCResult<T, E> {
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::ManagedArgBuffer<M> {
    type Unmanaged = super::heap::ArgBuffer;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::FunctionCall<M> {
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::BigFloat<M> {
    type Unmanaged = f64;
}

#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> HasUnmanaged for super::BigInt<M> {
    type Unmanaged = crate::codec::num_bigint::BigInt;
}

#[cfg(not(feature = "num-bigint"))]
impl<M: ManagedTypeApi> HasUnmanaged for super::BigInt<M> {
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::EllipticCurve<M> {
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::ManagedBuffer<M> {
    type Unmanaged = Vec<u8>;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::ManagedDecimal<M, super::NumDecimals> {
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi, DECIMALS: crate::typenum::Unsigned> HasUnmanaged
    for super::ManagedDecimal<M, super::ConstDecimals<DECIMALS>>
{
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::ManagedDecimalSigned<M, super::NumDecimals> {
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi, DECIMALS: crate::typenum::Unsigned> HasUnmanaged
    for super::ManagedDecimalSigned<M, super::ConstDecimals<DECIMALS>>
{
    type Unmanaged = Self;
}

impl<M, T> HasUnmanaged for super::ManagedAsyncCallResult<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbi,
{
    type Unmanaged = Self;
}

impl<M, T> HasUnmanaged for super::MultiValueEncoded<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbi + HasUnmanaged,
{
    type Unmanaged = MultiValueVec<T::Unmanaged>;
}

impl<M, T> HasUnmanaged for super::MultiValueEncodedCounted<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbi + HasUnmanaged + crate::codec::MultiValueConstLength,
{
    type Unmanaged = MultiValueVec<T::Unmanaged>;
}

impl<M, T> HasUnmanaged for super::MultiValueManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbi + HasUnmanaged + super::ManagedVecItem,
{
    type Unmanaged = MultiValueVec<T::Unmanaged>;
}

impl<M, T> HasUnmanaged for super::MultiValueManagedVecCounted<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbi + super::ManagedVecItem,
{
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::EsdtTokenPaymentMultiValue<M> {
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::EgldOrEsdtTokenPaymentMultiValue<M> {
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::PaymentMultiValue<M> {
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::ManagedAddress<M> {
    type Unmanaged = super::heap::Address;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::ManagedBufferReadToEnd<M> {
    type Unmanaged = Vec<u8>;
}

impl<M: ManagedTypeApi, const N: usize> HasUnmanaged for super::ManagedByteArray<M, N> {
    type Unmanaged = [u8; N];
}

impl<M, T> HasUnmanaged for super::ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbi + HasUnmanaged + super::ManagedType<M>,
{
    type Unmanaged = Option<T::Unmanaged>;
}

impl<M, T> HasUnmanaged for super::ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbi + HasUnmanaged + super::ManagedVecItem,
{
    type Unmanaged = Vec<T::Unmanaged>;
}

#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> HasUnmanaged for super::BigUint<M> {
    type Unmanaged = crate::codec::num_bigint::BigUint;
}

#[cfg(not(feature = "num-bigint"))]
impl<M: ManagedTypeApi> HasUnmanaged for super::BigUint<M> {
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::NonZeroBigUint<M> {
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::EsdtTokenIdentifier<M> {
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::EgldOrEsdtTokenIdentifier<M> {
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::FungiblePayment<M> {
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::Payment<M> {
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi> HasUnmanaged for super::TokenId<M> {
    type Unmanaged = Self;
}

impl<E: ErrorApi, const CAPACITY: usize> HasUnmanaged for super::SparseArray<E, CAPACITY> {
    type Unmanaged = Self;
}
