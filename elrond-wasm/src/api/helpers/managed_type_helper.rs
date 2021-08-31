use crate::{
    api::ManagedTypeApi,
    types::{BigUint, EllipticCurve, ManagedBuffer, TokenIdentifier},
};

pub struct ManagedTypeHelper<M: ManagedTypeApi> {
    api: M,
}

impl<M: ManagedTypeApi> ManagedTypeHelper<M> {
    pub(crate) fn new(api: M) -> Self {
        ManagedTypeHelper { api }
    }

    pub fn big_uint_zero(&self) -> BigUint<M> {
        BigUint::zero(self.api.clone())
    }

    pub fn big_uint_from<T: Into<u64>>(&self, value: T) -> BigUint<M> {
        BigUint::from_u64(self.api.clone(), value.into())
    }

    pub fn managed_buffer_empty(&self) -> ManagedBuffer<M> {
        ManagedBuffer::new_empty(self.api.clone())
    }

    pub fn managed_buffer_from(&self, bytes: &[u8]) -> ManagedBuffer<M> {
        ManagedBuffer::new_from_bytes(self.api.clone(), bytes)
    }

    pub fn elliptic_curve(&self, name: &str) -> EllipticCurve<M> {
        EllipticCurve::from_name(self.api.clone(), name)
    }

    pub fn elliptic_curve_from_bitsize(&self, bitsize: u32) -> Option<EllipticCurve<M>> {
        EllipticCurve::from_bitsize(self.api.clone(), bitsize)
    }

    pub fn token_identifier_egld(&self) -> TokenIdentifier<M> {
        TokenIdentifier::egld(self.api.clone())
    }
}
