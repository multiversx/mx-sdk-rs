use core::cmp::Ordering;

use super::{HandleTypeInfo, Sign};

/// Definition of the BigFloat type required by the API.
pub trait BigFloatApiImpl: HandleTypeInfo {
    fn bf_from_parts(
        &self,
        integral_part: i32,
        fractional_part: i32,
        exponent: i32,
    ) -> Self::BigFloatHandle;
    fn bf_from_frac(&self, numerator: i64, denominator: i64) -> Self::BigFloatHandle;
    fn bf_from_sci(&self, significand: i64, exponent: i64) -> Self::BigFloatHandle;

    fn bf_new_zero(&self) -> Self::BigFloatHandle {
        self.bf_from_frac(0, 1)
    }

    fn bf_add(&self, dest: Self::BigFloatHandle, x: Self::BigFloatHandle, y: Self::BigFloatHandle);
    fn bf_sub(&self, dest: Self::BigFloatHandle, x: Self::BigFloatHandle, y: Self::BigFloatHandle);
    fn bf_mul(&self, dest: Self::BigFloatHandle, x: Self::BigFloatHandle, y: Self::BigFloatHandle);
    fn bf_div(&self, dest: Self::BigFloatHandle, x: Self::BigFloatHandle, y: Self::BigFloatHandle);

    fn bf_abs(&self, dest: Self::BigFloatHandle, x: Self::BigFloatHandle);
    fn bf_neg(&self, dest: Self::BigFloatHandle, x: Self::BigFloatHandle);
    fn bf_cmp(&self, x: Self::BigFloatHandle, y: Self::BigFloatHandle) -> Ordering;
    fn bf_sign(&self, x: Self::BigFloatHandle) -> Sign;
    fn bf_clone(&self, dest: Self::BigFloatHandle, x: Self::BigFloatHandle);
    fn bf_sqrt(&self, dest: Self::BigFloatHandle, x: Self::BigFloatHandle);
    fn bf_pow(&self, dest: Self::BigFloatHandle, x: Self::BigFloatHandle, exp: i32);

    fn bf_floor(&self, dest: Self::BigIntHandle, x: Self::BigFloatHandle);
    fn bf_ceil(&self, dest: Self::BigIntHandle, x: Self::BigFloatHandle);
    fn bf_trunc(&self, dest: Self::BigIntHandle, x: Self::BigFloatHandle);

    fn bf_is_bi(&self, x: Self::BigFloatHandle) -> bool;
    fn bf_set_i64(&self, dest: Self::BigFloatHandle, value: i64);
    fn bf_set_bi(&self, dest: Self::BigFloatHandle, bi: Self::BigIntHandle);

    fn bf_get_const_pi(&self, dest: Self::BigFloatHandle);
    fn bf_get_const_e(&self, dest: Self::BigFloatHandle);
}
