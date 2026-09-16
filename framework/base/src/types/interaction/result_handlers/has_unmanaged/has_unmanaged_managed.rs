use alloc::vec::Vec;

use crate::{
    api::{ManagedTypeApi, UnmanagedApi},
    typenum::Unsigned,
    types::{
        ConstDecimals, EllipticCurve, ManagedAddress, ManagedBuffer, ManagedBufferReadToEnd,
        ManagedByteArray, ManagedDecimal, ManagedDecimalSigned, ManagedOption, ManagedType,
        ManagedVec, ManagedVecItem, NumDecimals,
    },
};

use crate::types::HasUnmanaged;

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
    type Unmanaged = crate::types::heap::Address;
}

impl<M, T> HasUnmanaged for ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + HasUnmanaged,
{
    type Unmanaged = Option<T::Unmanaged>;
}

impl<M: UnmanagedApi> HasUnmanaged for EllipticCurve<M> {
    type Unmanaged = Self;
}

impl<M: UnmanagedApi> HasUnmanaged for ManagedDecimal<M, NumDecimals> {
    type Unmanaged = Self;
}

impl<M: UnmanagedApi, DECIMALS: Unsigned> HasUnmanaged
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    type Unmanaged = Self;
}

impl<M: UnmanagedApi> HasUnmanaged for ManagedDecimalSigned<M, NumDecimals> {
    type Unmanaged = Self;
}

impl<M: UnmanagedApi, DECIMALS: Unsigned> HasUnmanaged
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
    type Unmanaged = Self;
}
