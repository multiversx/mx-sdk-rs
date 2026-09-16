use multiversx_sc::{
    api::ManagedTypeApi,
    typenum::Unsigned,
    types::{
        ConstDecimals, EllipticCurve, ManagedAddress, ManagedBuffer, ManagedBufferReadToEnd,
        ManagedByteArray, ManagedDecimal, ManagedDecimalSigned, ManagedOption, ManagedType,
        ManagedVec, ManagedVecItem, NumDecimals,
    },
};

use crate::{api::StaticApi, facade::result_handlers::HasUnmanaged};

impl<M: ManagedTypeApi> HasUnmanaged for ManagedBuffer<M> {
    type Unmanaged = Vec<u8>;
}

impl<M: ManagedTypeApi> HasUnmanaged for ManagedBufferReadToEnd<M> {
    type Unmanaged = Vec<u8>;
}

impl<M, T> HasUnmanaged for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + HasUnmanaged,
{
    type Unmanaged = Vec<T::Unmanaged>;
}

impl<M, const N: usize> HasUnmanaged for ManagedByteArray<M, N>
where
    M: ManagedTypeApi,
{
    type Unmanaged = [u8; N];
}

impl<M> HasUnmanaged for ManagedAddress<M>
where
    M: ManagedTypeApi,
{
    type Unmanaged = multiversx_sc::types::heap::Address;
}

impl<M, T> HasUnmanaged for ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + HasUnmanaged,
{
    type Unmanaged = Option<T::Unmanaged>;
}

impl<M: ManagedTypeApi> HasUnmanaged for EllipticCurve<M> {
    type Unmanaged = EllipticCurve<StaticApi>;
}

impl<M: ManagedTypeApi> HasUnmanaged for ManagedDecimal<M, NumDecimals> {
    type Unmanaged = ManagedDecimal<StaticApi, NumDecimals>;
}

impl<M: ManagedTypeApi, DECIMALS: Unsigned> HasUnmanaged
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    type Unmanaged = ManagedDecimal<StaticApi, ConstDecimals<DECIMALS>>;
}

impl<M: ManagedTypeApi> HasUnmanaged for ManagedDecimalSigned<M, NumDecimals> {
    type Unmanaged = ManagedDecimalSigned<StaticApi, NumDecimals>;
}

impl<M: ManagedTypeApi, DECIMALS: Unsigned> HasUnmanaged
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
    type Unmanaged = ManagedDecimalSigned<StaticApi, ConstDecimals<DECIMALS>>;
}
