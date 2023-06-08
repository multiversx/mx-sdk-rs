use core::cmp::Ordering;

use crate::api::{BigFloatApiImpl, Sign};

impl BigFloatApiImpl for super::UncallableApi {
    fn bf_from_parts(
        &self,
        _integral_part: i32,
        _fractional_part: i32,
        _exponent: i32,
    ) -> Self::BigFloatHandle {
        unreachable!()
    }
    fn bf_from_frac(&self, _numerator: i64, _denominator: i64) -> Self::BigFloatHandle {
        unreachable!()
    }
    fn bf_from_sci(&self, _significand: i64, _exponent: i64) -> Self::BigFloatHandle {
        unreachable!()
    }

    fn bf_add(
        &self,
        _dest: Self::BigFloatHandle,
        _x: Self::BigFloatHandle,
        _y: Self::BigFloatHandle,
    ) {
        unreachable!()
    }
    fn bf_sub(
        &self,
        _dest: Self::BigFloatHandle,
        _x: Self::BigFloatHandle,
        _y: Self::BigFloatHandle,
    ) {
        unreachable!()
    }
    fn bf_mul(
        &self,
        _dest: Self::BigFloatHandle,
        _x: Self::BigFloatHandle,
        _y: Self::BigFloatHandle,
    ) {
        unreachable!()
    }
    fn bf_div(
        &self,
        _dest: Self::BigFloatHandle,
        _x: Self::BigFloatHandle,
        _y: Self::BigFloatHandle,
    ) {
        unreachable!()
    }

    fn bf_abs(&self, _dest: Self::BigFloatHandle, _x: Self::BigFloatHandle) {
        unreachable!()
    }
    fn bf_neg(&self, _dest: Self::BigFloatHandle, _x: Self::BigFloatHandle) {
        unreachable!()
    }
    fn bf_cmp(&self, _x: Self::BigFloatHandle, _y: Self::BigFloatHandle) -> Ordering {
        unreachable!()
    }
    fn bf_sign(&self, _x: Self::BigFloatHandle) -> Sign {
        unreachable!()
    }
    fn bf_clone(&self, _dest: Self::BigFloatHandle, _x: Self::BigFloatHandle) {
        unreachable!()
    }
    fn bf_sqrt(&self, _dest: Self::BigFloatHandle, _x: Self::BigFloatHandle) {
        unreachable!()
    }
    fn bf_pow(&self, _dest: Self::BigFloatHandle, _x: Self::BigFloatHandle, _exp: i32) {
        unreachable!()
    }

    fn bf_floor(&self, _dest: Self::BigIntHandle, _x: Self::BigFloatHandle) {
        unreachable!()
    }
    fn bf_ceil(&self, _dest: Self::BigIntHandle, _x: Self::BigFloatHandle) {
        unreachable!()
    }
    fn bf_trunc(&self, _dest: Self::BigIntHandle, _x: Self::BigFloatHandle) {
        unreachable!()
    }

    fn bf_is_bi(&self, _x: Self::BigFloatHandle) -> bool {
        unreachable!()
    }

    fn bf_set_i64(&self, _dest: Self::BigFloatHandle, _value: i64) {
        unreachable!()
    }

    fn bf_set_bi(&self, _dest: Self::BigFloatHandle, _bi: Self::BigIntHandle) {
        unreachable!()
    }

    fn bf_get_const_pi(&self, _dest: Self::BigFloatHandle) {
        unreachable!()
    }
    fn bf_get_const_e(&self, _dest: Self::BigFloatHandle) {
        unreachable!()
    }
}
