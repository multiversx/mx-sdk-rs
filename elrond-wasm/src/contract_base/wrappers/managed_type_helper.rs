use crate::{
    api::ManagedTypeApi,
    types::{
        BigFloat, BigInt, BigUint, EllipticCurve, ManagedAddress, ManagedBuffer,
        ManagedMultiResultVec, ManagedVec, ManagedVecItem, TokenIdentifier,
    },
};

pub struct ManagedTypeHelper<M: ManagedTypeApi> {
    _api: M,
}

impl<M: ManagedTypeApi> ManagedTypeHelper<M> {
    pub(crate) fn new(_api: M) -> Self {
        ManagedTypeHelper { _api }
    }

    #[inline]
    pub fn big_uint_zero(&self) -> BigUint<M> {
        BigUint::zero()
    }

    #[inline]
    pub fn big_uint_from<T: Into<BigUint<M>>>(&self, value: T) -> BigUint<M> {
        value.into()
    }

    #[inline]
    pub fn big_int_zero(&self) -> BigInt<M> {
        BigInt::zero()
    }

    #[inline]
    pub fn big_int_from<T: Into<BigInt<M>>>(&self, value: T) -> BigInt<M> {
        value.into()
    }

    #[inline]
    pub fn big_float_from<T: ManagedInto<M, BigFloat<M>>>(&self, value: T) -> BigFloat<M> {
        value.managed_into(self.api.clone())
    }

    #[inline]
    pub fn big_float_zero(&self) -> BigFloat<M> {
        BigFloat::zero(self.api.clone())
    }

    #[inline]
    pub fn big_float_from_parts(
        &self,
        integral_part_value: i32,
        fractional_part_value: i32,
        exponent_value: i32,
    ) -> BigFloat<M> {
        BigFloat::from_parts(
            self.api.clone(),
            integral_part_value,
            fractional_part_value,
            exponent_value,
        )
    }

    #[inline]
    pub fn big_float_from_frac(&self, numerator_value: i64, denominator_value: i64) -> BigFloat<M> {
        BigFloat::from_frac(self.api.clone(), numerator_value, denominator_value)
    }

    #[inline]
    pub fn big_float_from_sci(&self, significand_value: i64, exponent_value: i32) -> BigFloat<M> {
        BigFloat::from_sci(self.api.clone(), significand_value, exponent_value)
    }

    #[inline]
    pub fn managed_buffer_new(&self) -> ManagedBuffer<M> {
        ManagedBuffer::new()
    }

    #[inline]
    pub fn managed_buffer_from<T: Into<ManagedBuffer<M>>>(&self, value: T) -> ManagedBuffer<M> {
        value.into()
    }

    #[inline]
    pub fn managed_vec_new<T: ManagedVecItem>(&self) -> ManagedVec<M, T> {
        ManagedVec::new()
    }

    #[inline]
    pub fn managed_vec_from_single_item<T: ManagedVecItem>(&self, item: T) -> ManagedVec<M, T> {
        ManagedVec::from_single_item(item)
    }

    #[inline]
    pub fn managed_vec_from<T: ManagedVecItem, V: Into<ManagedVec<M, T>>>(
        &self,
        value: V,
    ) -> ManagedVec<M, T> {
        value.into()
    }

    #[inline]
    pub fn managed_multi_result_vec_new<T>(&self) -> ManagedMultiResultVec<M, T> {
        ManagedMultiResultVec::new()
    }

    #[inline]
    pub fn elliptic_curve(&self, name: &str) -> EllipticCurve<M> {
        EllipticCurve::from_name(name)
    }

    #[inline]
    pub fn elliptic_curve_from_bitsize(&self, bitsize: u32) -> Option<EllipticCurve<M>> {
        EllipticCurve::from_bitsize(bitsize)
    }

    #[inline]
    pub fn token_identifier_egld(&self) -> TokenIdentifier<M> {
        TokenIdentifier::egld()
    }

    #[inline]
    pub fn token_identifier_from<T: Into<TokenIdentifier<M>>>(
        &self,
        value: T,
    ) -> TokenIdentifier<M> {
        value.into()
    }

    #[inline]
    pub fn managed_address_zero(&self) -> ManagedAddress<M> {
        ManagedAddress::zero()
    }

    #[inline]
    pub fn managed_address_from<T: Into<ManagedAddress<M>>>(&self, value: T) -> ManagedAddress<M> {
        value.into()
    }
}
