use crate::{
    api::ManagedTypeApi,
    types::{
        BigFloat, BigInt, BigUint, EllipticCurve, ManagedAddress, ManagedBuffer, ManagedInto,
        ManagedMultiResultVec, ManagedVec, ManagedVecItem, TokenIdentifier,
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
    pub fn big_float_from_sci(&self, significand_value: i64, exponent_value: i64) -> BigFloat<M> {
        BigFloat::from_frac(self.api.clone(), significand_value, exponent_value)
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
    pub fn managed_vec_from_single_item<T: ManagedVecItem<M>>(&self, item: T) -> ManagedVec<M, T> {
        ManagedVec::from_single_item(self.api.clone(), item)
    }

    #[inline]
    pub fn managed_vec_from<T: ManagedVecItem<M>, V: ManagedInto<M, ManagedVec<M, T>>>(
        &self,
        value: V,
    ) -> ManagedVec<M, T> {
        value.managed_into(self.api.clone())
    }

    #[inline]
    pub fn managed_multi_result_vec_new<T>(&self) -> ManagedMultiResultVec<M, T> {
        ManagedMultiResultVec::new(self.api.clone())
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
    pub fn token_identifier_from<T: ManagedInto<M, TokenIdentifier<M>>>(
        &self,
        value: T,
    ) -> TokenIdentifier<M> {
        value.managed_into(self.api.clone())
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
