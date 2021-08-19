use crate::types::{BigUint, EllipticCurve, ManagedBuffer};

use super::ManagedTypeApi;

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

    pub fn big_uint_from(&self, value: u64) -> BigUint<M> {
        BigUint::from_u64(value, self.api.clone())
    }

    pub fn managed_buffer_empty(&self) -> ManagedBuffer<M> {
        ManagedBuffer::new_empty(self.api.clone())
    }

    pub fn managed_buffer_from(&self, bytes: &[u8]) -> ManagedBuffer<M> {
        ManagedBuffer::new_from_bytes(self.api.clone(), bytes)
    }

    pub fn elliptic_curve(&self, name: &str) -> EllipticCurve<M> {
        EllipticCurve::from_name(name, self.api.clone())
    }

    pub fn elliptic_curve_from_bitsize(&self, bitsize: u32) -> Option<EllipticCurve<M>> {
        EllipticCurve::from_bitsize(bitsize, self.api.clone())
    }
}
