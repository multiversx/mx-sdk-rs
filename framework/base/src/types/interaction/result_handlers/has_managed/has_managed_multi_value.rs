use crate::{
    abi::TypeAbi,
    api::ManagedTypeApi,
    codec::{MultiValueConstLength, multi_types::*},
    types::{
        EgldOrEsdtTokenPaymentMultiValue, EsdtTokenPaymentMultiValue, ManagedAsyncCallResult,
        ManagedVecItem, MultiValueEncoded, MultiValueEncodedCounted, MultiValueManagedVec,
        MultiValueManagedVecCounted, PaymentMultiValue,
    },
};

use crate::types::HasManaged;

impl<M: ManagedTypeApi> HasManaged<M> for IgnoreValue {
    type Managed = Self;
}

impl<M: ManagedTypeApi, T: HasManaged<M>> HasManaged<M> for MultiValueVec<T> {
    type Managed = MultiValueEncoded<M, T::Managed>;
}

impl<M: ManagedTypeApi, T: HasManaged<M>> HasManaged<M> for OptionalValue<T> {
    type Managed = OptionalValue<T::Managed>;
}

macro_rules! multi_value_impls {
    ($(($mval_struct:ident $($t:ident)+) )+) => {
        $(
            impl<M: ManagedTypeApi, $($t),+> HasManaged<M> for $mval_struct<$($t,)+>
            where
                $($t: HasManaged<M>,)+
            {
                type Managed = $mval_struct<$($t::Managed,)+>;
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

impl<M: ManagedTypeApi, T: TypeAbi> HasManaged<M> for ManagedAsyncCallResult<M, T> {
    type Managed = Self;
}

impl<M: ManagedTypeApi> HasManaged<M> for EgldOrEsdtTokenPaymentMultiValue<M> {
    type Managed = Self;
}

impl<M: ManagedTypeApi> HasManaged<M> for EsdtTokenPaymentMultiValue<M> {
    type Managed = Self;
}

impl<M: ManagedTypeApi, T: HasManaged<M>> HasManaged<M> for MultiValueEncoded<M, T> {
    type Managed = MultiValueEncoded<M, T::Managed>;
}

impl<M: ManagedTypeApi, T> HasManaged<M> for MultiValueEncodedCounted<M, T>
where
    T: HasManaged<M> + MultiValueConstLength,
    T::Managed: MultiValueConstLength,
{
    type Managed = MultiValueEncodedCounted<M, T::Managed>;
}

impl<M: ManagedTypeApi, T> HasManaged<M> for MultiValueManagedVec<M, T>
where
    T: ManagedVecItem + HasManaged<M>,
    T::Managed: ManagedVecItem,
{
    type Managed = MultiValueManagedVec<M, T::Managed>;
}

impl<M: ManagedTypeApi, T: ManagedVecItem + TypeAbi> HasManaged<M>
    for MultiValueManagedVecCounted<M, T>
{
    type Managed = Self;
}

impl<M: ManagedTypeApi> HasManaged<M> for PaymentMultiValue<M> {
    type Managed = Self;
}
