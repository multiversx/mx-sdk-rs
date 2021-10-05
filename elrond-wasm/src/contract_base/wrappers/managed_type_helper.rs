use crate::{
    api::ManagedTypeApi,
    types::{
        BigInt, BigUint, EllipticCurve, ManagedAddress, ManagedBuffer, ManagedInto, ManagedVec,
        ManagedVecItem, TokenIdentifier,
    },
};

pub struct ManagedTypeHelper<M: ManagedTypeApi> {
    api: M,
}

impl<M: ManagedTypeApi> ManagedTypeHelper<M> {
    pub(crate) fn new(api: M) -> Self {
        ManagedTypeHelper { api }
    }

    #[inline]
    pub fn big_uint_zero(&self) -> BigUint<M> {
        BigUint::zero(self.api.clone())
    }

    #[inline]
    pub fn big_uint_from<T: ManagedInto<M, BigUint<M>>>(&self, value: T) -> BigUint<M> {
        value.managed_into(self.api.clone())
    }

    #[inline]
    pub fn big_int_zero(&self) -> BigInt<M> {
        BigInt::zero(self.api.clone())
    }

    #[inline]
    pub fn big_int_from<T: ManagedInto<M, BigInt<M>>>(&self, value: T) -> BigInt<M> {
        value.managed_into(self.api.clone())
    }

    #[inline]
    pub fn managed_buffer_new(&self) -> ManagedBuffer<M> {
        ManagedBuffer::new(self.api.clone())
    }

    #[inline]
    pub fn managed_buffer_from<T: ManagedInto<M, ManagedBuffer<M>>>(
        &self,
        value: T,
    ) -> ManagedBuffer<M> {
        value.managed_into(self.api.clone())
    }

    #[inline]
    pub fn managed_vec_new<T: ManagedVecItem<M>>(&self) -> ManagedVec<M, T> {
        ManagedVec::new(self.api.clone())
    }

    #[inline]
    pub fn managed_vec_from<T: ManagedVecItem<M>, V: ManagedInto<M, ManagedVec<M, T>>>(
        &self,
        value: V,
    ) -> ManagedVec<M, T> {
        value.managed_into(self.api.clone())
    }

    #[inline]
    pub fn elliptic_curve(&self, name: &str) -> EllipticCurve<M> {
        EllipticCurve::from_name(self.api.clone(), name)
    }

    #[inline]
    pub fn elliptic_curve_from_bitsize(&self, bitsize: u32) -> Option<EllipticCurve<M>> {
        EllipticCurve::from_bitsize(self.api.clone(), bitsize)
    }

    #[inline]
    pub fn token_identifier_egld(&self) -> TokenIdentifier<M> {
        TokenIdentifier::egld(self.api.clone())
    }

    #[inline]
    pub fn token_identifier_from_esdt_bytes<B>(&self, bytes: B) -> TokenIdentifier<M>
    where
        B: ManagedInto<M, ManagedBuffer<M>>,
    {
        TokenIdentifier::from_esdt_bytes(self.api.clone(), bytes)
    }

    #[inline]
    pub fn managed_address_zero(&self) -> ManagedAddress<M> {
        ManagedAddress::zero(self.api.clone())
    }

    #[inline]
    pub fn managed_address_from<T: ManagedInto<M, ManagedAddress<M>>>(
        &self,
        value: T,
    ) -> ManagedAddress<M> {
        value.managed_into(self.api.clone())
    }
}
