use crate::{
    abi::TypeAbi,
    api::{ManagedTypeApi, UnmanagedApi},
    codec::{MultiValueConstLength, multi_types::*},
    types::{
        EgldOrEsdtTokenPaymentMultiValue, EsdtTokenPaymentMultiValue, ManagedAsyncCallResult,
        ManagedVecItem, MultiValueEncoded, MultiValueEncodedCounted, MultiValueManagedVec,
        MultiValueManagedVecCounted, PaymentMultiValue,
    },
};

use crate::types::HasUnmanaged;

impl HasUnmanaged for IgnoreValue {
    type Unmanaged = Self;
}

impl<T: HasUnmanaged> HasUnmanaged for MultiValueVec<T> {
    type Unmanaged = MultiValueVec<T::Unmanaged>;
}

impl<T: HasUnmanaged> HasUnmanaged for OptionalValue<T> {
    type Unmanaged = OptionalValue<T::Unmanaged>;
}

macro_rules! multi_value_impls {
    ($(($mval_struct:ident $($t:ident)+) )+) => {
        $(
            impl<$($t),+> HasUnmanaged for $mval_struct<$($t,)+>
            where
                $($t: HasUnmanaged,)+
            {
                type Unmanaged = $mval_struct<$($t::Unmanaged,)+>;
            }
        )+
    }
}

multi_value_impls! {
    (MultiValue2 T0 T1)
    (MultiValue3 T0 T1 T2)
    (MultiValue4 T0 T1 T2 T3)
    (MultiValue5 T0 T1 T2 T3 T4)
    (MultiValue6 T0 T1 T2 T3 T4 T5)
    (MultiValue7 T0 T1 T2 T3 T4 T5 T6)
    (MultiValue8 T0 T1 T2 T3 T4 T5 T6 T7)
    (MultiValue9 T0 T1 T2 T3 T4 T5 T6 T7 T8)
    (MultiValue10 T0 T1 T2 T3 T4 T5 T6 T7 T8 T9)
    (MultiValue11 T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10)
    (MultiValue12 T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11)
    (MultiValue13 T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12)
    (MultiValue14 T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13)
    (MultiValue15 T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14)
    (MultiValue16 T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15)
}

impl<M, T> HasUnmanaged for ManagedAsyncCallResult<M, T>
where
    M: UnmanagedApi,
    T: TypeAbi,
{
    type Unmanaged = Self;
}

impl<M> HasUnmanaged for EgldOrEsdtTokenPaymentMultiValue<M>
where
    M: UnmanagedApi,
{
    type Unmanaged = Self;
}

impl<M> HasUnmanaged for EsdtTokenPaymentMultiValue<M>
where
    M: UnmanagedApi,
{
    type Unmanaged = Self;
}

impl<M, T> HasUnmanaged for MultiValueEncoded<M, T>
where
    M: ManagedTypeApi,
    T: HasUnmanaged,
{
    type Unmanaged = MultiValueVec<T::Unmanaged>;
}

impl<M, T> HasUnmanaged for MultiValueEncodedCounted<M, T>
where
    M: ManagedTypeApi,
    T: HasUnmanaged + MultiValueConstLength,
{
    type Unmanaged = MultiValueVec<T::Unmanaged>;
}

impl<M, T> HasUnmanaged for MultiValueManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + HasUnmanaged,
{
    type Unmanaged = MultiValueVec<T::Unmanaged>;
}

impl<M, T> HasUnmanaged for MultiValueManagedVecCounted<M, T>
where
    M: UnmanagedApi,
    T: ManagedVecItem + TypeAbi,
{
    type Unmanaged = Self;
}

impl<M> HasUnmanaged for PaymentMultiValue<M>
where
    M: UnmanagedApi,
{
    type Unmanaged = Self;
}
