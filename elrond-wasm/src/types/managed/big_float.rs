use super::{ManagedBuffer, ManagedType};
use crate::{
    api::{Handle, ManagedTypeApi, Sign},
    types::{BigInt, BigUint},
};
use elrond_codec::TryStaticCast;

#[derive(Debug)]
pub struct BigFloat<M: ManagedTypeApi> {
    pub(crate) handle: Handle,
    pub(crate) api: M,
}

impl<M: ManagedTypeApi> ManagedType<M> for BigFloat<M> {
    #[doc(hidden)]
    fn from_raw_handle(api: M, raw_handle: Handle) -> Self {
        BigFloat {
            handle: raw_handle,
            api,
        }
    }

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle {
        self.handle
    }

    #[inline]
    fn type_manager(&self) -> M {
        self.api.clone()
    }
}

impl<M: ManagedTypeApi> TryStaticCast for BigFloat<M> {}

impl<M: ManagedTypeApi> From<&ManagedBuffer<M>> for BigFloat<M> {
    fn from(item: &ManagedBuffer<M>) -> Self {
        BigFloat::from_buffer(item)
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for BigFloat<M> {
    fn from(item: ManagedBuffer<M>) -> Self {
        BigFloat::from_buffer(&item)
    }
}

impl<M: ManagedTypeApi> From<&BigUint<M>> for BigFloat<M> {
    fn from(item: &BigUint<M>) -> Self {
        BigFloat::from_big_uint(item)
    }
}

impl<M: ManagedTypeApi> From<BigUint<M>> for BigFloat<M> {
    fn from(item: BigUint<M>) -> Self {
        BigFloat::from_big_uint(&item)
    }
}

impl<M: ManagedTypeApi> From<&BigInt<M>> for BigFloat<M> {
    fn from(item: &BigInt<M>) -> Self {
        BigFloat::from_big_int(item)
    }
}

impl<M: ManagedTypeApi> From<BigInt<M>> for BigFloat<M> {
    fn from(item: BigInt<M>) -> Self {
        BigFloat::from_big_int(&item)
    }
}

impl<M: ManagedTypeApi> BigFloat<M> {
    #[inline]
    pub fn from_big_uint(big_uint: &BigUint<M>) -> Self {
        let new_bf_handle = big_uint.api.bf_new_zero();
        big_uint.api.bf_set_bi(new_bf_handle, big_uint.handle);
        BigFloat {
            handle: new_bf_handle,
            api: big_uint.api.clone(),
        }
    }

    #[inline]
    pub fn from_big_int(big_int: &BigInt<M>) -> Self {
        let new_bf_handle = big_int.api.bf_new_zero();
        big_int.api.bf_set_bi(new_bf_handle, big_int.handle);
        BigFloat {
            handle: new_bf_handle,
            api: big_int.api.clone(),
        }
    }

    #[inline]
    pub fn from_parts(
        &self,
        integral_part_value: i32,
        fractional_part_value: i32,
        exponent_value: i32,
    ) -> Self {
        let new_bf_handle =
            self.api
                .bf_from_parts(integral_part_value, fractional_part_value, exponent_value);
        BigFloat {
            handle: new_bf_handle,
            api: self.api.clone(),
        }
    }

    #[inline]
    pub fn from_frac(&self, numerator_value: i64, denominator_value: i64) -> Self {
        let new_bf_handle = self.api.bf_from_frac(numerator_value, denominator_value);
        BigFloat {
            handle: new_bf_handle,
            api: self.api.clone(),
        }
    }

    #[inline]
    pub fn from_sci(&self, significand_value: i64, exponent_value: i32) -> Self {
        let new_bf_handle = self
            .api
            .bf_from_sci(significand_value, exponent_value as i64);
        BigFloat {
            handle: new_bf_handle,
            api: self.api.clone(),
        }
    }
}

impl<M: ManagedTypeApi> BigFloat<M> {
    #[inline]
    pub fn zero(api: M) -> Self {
        BigFloat {
            handle: api.bf_new_zero(),
            api,
        }
    }

    #[inline]
    pub fn from_buffer(managed_buffer: &ManagedBuffer<M>) -> Self {
        BigFloat {
            handle: managed_buffer.api.mb_to_big_float(managed_buffer.handle),
            api: managed_buffer.api.clone(),
        }
    }

    #[inline]
    pub fn to_buffer(&self) -> ManagedBuffer<M> {
        ManagedBuffer {
            handle: self.api.mb_from_big_float(self.handle),
            api: self.api.clone(),
        }
    }
}

impl<M: ManagedTypeApi> BigFloat<M> {
    #[inline]
    pub fn sqrt(&self) -> Self {
        let new_handle = self.api.bf_new_zero();
        self.api.bf_sqrt(new_handle, self.handle);
        BigFloat {
            handle: new_handle,
            api: self.api.clone(),
        }
    }

    pub fn pow(&self, exp: u32) -> Self {
        let new_handle = self.api.bf_new_zero();
        self.api.bf_pow(new_handle, self.handle, exp as i32);
        BigFloat {
            handle: new_handle,
            api: self.api.clone(),
        }
    }

    /// Returns the sign of the `BigFloat` as a `Sign`.
    pub fn sign(&self) -> Sign {
        match self.api.bf_sign(self.handle) {
            crate::api::Sign::Plus => Sign::Plus,
            crate::api::Sign::NoSign => Sign::NoSign,
            crate::api::Sign::Minus => Sign::Minus,
        }
    }

    /// Returns the magnitude of the `BigFloat`
    pub fn magnitude(&self) -> BigFloat<M> {
        let result = self.api.bf_new_zero();
        self.api.bf_abs(result, self.handle);
        BigFloat {
            handle: result,
            api: self.api.clone(),
        }
    }
}

impl<M: ManagedTypeApi> Clone for BigFloat<M> {
    fn clone(&self) -> Self {
        let new_handle = self.api.bf_new_zero();
        self.api.bf_clone(new_handle, self.handle);
        BigFloat {
            handle: new_handle,
            api: self.api.clone(),
        }
    }
}
