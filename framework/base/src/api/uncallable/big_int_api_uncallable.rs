use core::cmp::Ordering;

use crate::api::{BigIntApiImpl, Sign};

impl BigIntApiImpl for super::UncallableApi {
    fn bi_new(&self, _value: i64) -> Self::BigIntHandle {
        unreachable!()
    }

    fn bi_set_int64(&self, _destination: Self::BigIntHandle, _value: i64) {
        unreachable!()
    }

    fn bi_to_i64(&self, _reference: Self::BigIntHandle) -> Option<i64> {
        unreachable!()
    }

    fn bi_add(&self, _dest: Self::BigIntHandle, _x: Self::BigIntHandle, _y: Self::BigIntHandle) {
        unreachable!()
    }

    fn bi_sub(&self, _dest: Self::BigIntHandle, _x: Self::BigIntHandle, _y: Self::BigIntHandle) {
        unreachable!()
    }

    fn bi_mul(&self, _dest: Self::BigIntHandle, _x: Self::BigIntHandle, _y: Self::BigIntHandle) {
        unreachable!()
    }

    fn bi_t_div(&self, _dest: Self::BigIntHandle, _x: Self::BigIntHandle, _y: Self::BigIntHandle) {
        unreachable!()
    }

    fn bi_t_mod(&self, _dest: Self::BigIntHandle, _x: Self::BigIntHandle, _y: Self::BigIntHandle) {
        unreachable!()
    }

    fn bi_abs(&self, _dest: Self::BigIntHandle, _x: Self::BigIntHandle) {
        unreachable!()
    }

    fn bi_neg(&self, _dest: Self::BigIntHandle, _x: Self::BigIntHandle) {
        unreachable!()
    }

    fn bi_sign(&self, _x: Self::BigIntHandle) -> Sign {
        unreachable!()
    }

    fn bi_cmp(&self, _x: Self::BigIntHandle, _y: Self::BigIntHandle) -> Ordering {
        unreachable!()
    }

    fn bi_sqrt(&self, _dest: Self::BigIntHandle, _x: Self::BigIntHandle) {
        unreachable!()
    }

    fn bi_pow(&self, _dest: Self::BigIntHandle, _x: Self::BigIntHandle, _y: Self::BigIntHandle) {
        unreachable!()
    }

    fn bi_log2(&self, _x: Self::BigIntHandle) -> u32 {
        unreachable!()
    }

    fn bi_and(&self, _dest: Self::BigIntHandle, _x: Self::BigIntHandle, _y: Self::BigIntHandle) {
        unreachable!()
    }

    fn bi_or(&self, _dest: Self::BigIntHandle, _x: Self::BigIntHandle, _y: Self::BigIntHandle) {
        unreachable!()
    }

    fn bi_xor(&self, _dest: Self::BigIntHandle, _x: Self::BigIntHandle, _y: Self::BigIntHandle) {
        unreachable!()
    }

    fn bi_shr(&self, _dest: Self::BigIntHandle, _x: Self::BigIntHandle, _bits: usize) {
        unreachable!()
    }

    fn bi_shl(&self, _dest: Self::BigIntHandle, _x: Self::BigIntHandle, _bits: usize) {
        unreachable!()
    }

    fn bi_to_string(&self, _bi_handle: Self::BigIntHandle, _str_handle: Self::ManagedBufferHandle) {
        unreachable!()
    }
}
