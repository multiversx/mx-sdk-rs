use crate::{
    abi::TypeAbi,
    api::ManagedTypeApi,
    typenum::Unsigned,
    types::{
        ConstDecimals, EllipticCurve, ManagedAddress, ManagedBuffer, ManagedBufferReadToEnd,
        ManagedByteArray, ManagedDecimal, ManagedDecimalSigned, ManagedOption, ManagedType,
        ManagedVec, ManagedVecItem, NumDecimals,
    },
};

use crate::types::HasManaged;

impl<M: ManagedTypeApi> HasManaged<M> for ManagedBuffer<M> {
    type Managed = Self;
}

impl<M: ManagedTypeApi> HasManaged<M> for ManagedBufferReadToEnd<M> {
    type Managed = Self;
}

impl<M: ManagedTypeApi, T: ManagedVecItem + TypeAbi> HasManaged<M> for ManagedVec<M, T> {
    type Managed = Self;
}

impl<M: ManagedTypeApi, const N: usize> HasManaged<M> for ManagedByteArray<M, N> {
    type Managed = Self;
}

impl<M: ManagedTypeApi> HasManaged<M> for ManagedAddress<M> {
    type Managed = Self;
}

impl<M: ManagedTypeApi, T: ManagedType<M> + TypeAbi> HasManaged<M> for ManagedOption<M, T> {
    type Managed = Self;
}

impl<M: ManagedTypeApi> HasManaged<M> for EllipticCurve<M> {
    type Managed = Self;
}

impl<M: ManagedTypeApi> HasManaged<M> for ManagedDecimal<M, NumDecimals> {
    type Managed = Self;
}

impl<M: ManagedTypeApi, DECIMALS: Unsigned> HasManaged<M>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    type Managed = Self;
}

impl<M: ManagedTypeApi> HasManaged<M> for ManagedDecimalSigned<M, NumDecimals> {
    type Managed = Self;
}

impl<M: ManagedTypeApi, DECIMALS: Unsigned> HasManaged<M>
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
    type Managed = Self;
}
