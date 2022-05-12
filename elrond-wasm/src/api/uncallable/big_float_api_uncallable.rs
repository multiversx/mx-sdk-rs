use core::cmp::Ordering;

use crate::api::{BigFloatApi, Handle, Sign};

impl BigFloatApi for super::UncallableApi {
    fn bf_from_parts(&self, _integral_part: i32, _fractional_part: i32, _exponent: i32) -> Handle {
        unreachable!()
    }
    fn bf_from_frac(&self, _numerator: i64, _denominator: i64) -> Handle {
        unreachable!()
    }
    fn bf_from_sci(&self, _significand: i64, _exponent: i64) -> Handle {
        unreachable!()
    }

    fn bf_add(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }
    fn bf_sub(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }
    fn bf_mul(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }
    fn bf_div(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn bf_abs(&self, _dest: Handle, _x: Handle) {
        unreachable!()
    }
    fn bf_neg(&self, _dest: Handle, _x: Handle) {
        unreachable!()
    }
    fn bf_cmp(&self, _x: Handle, _y: Handle) -> Ordering {
        unreachable!()
    }
    fn bf_sign(&self, _x: Handle) -> Sign {
        unreachable!()
    }
    fn bf_clone(&self, _dest: Handle, _x: Handle) {
        unreachable!()
    }
    fn bf_sqrt(&self, _dest: Handle, _x: Handle) {
        unreachable!()
    }
    fn bf_pow(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn bf_floor(&self, _dest: Handle, _x: Handle) {
        unreachable!()
    }
    fn bf_ceil(&self, _dest: Handle, _x: Handle) {
        unreachable!()
    }
    fn bf_trunc(&self, _dest: Handle, _x: Handle) {
        unreachable!()
    }

    fn bf_is_bi(&self, _x: Handle) -> bool {
        unreachable!()
    }
    fn bf_set_i64(&self, _dest: Handle, _value: i64) {
        unreachable!()
    }
    fn bf_set_bi(&self, _dest: Handle, _bi: Handle) {
        unreachable!()
    }

    fn bf_get_const_pi(&self, _dest: Handle) {
        unreachable!()
    }
    fn bf_get_const_e(&self, _dest: Handle) {
        unreachable!()
    }
}
