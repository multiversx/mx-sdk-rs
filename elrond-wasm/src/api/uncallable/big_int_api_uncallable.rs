use core::cmp::Ordering;

use crate::{
    api::{BigIntApi, Handle, Sign},
    types::BoxedBytes,
};

impl BigIntApi for super::UncallableApi {
    fn bi_new(&self, _value: i64) -> Handle {
        unreachable!()
    }

    fn bi_unsigned_byte_length(&self, _x: Handle) -> usize {
        unreachable!()
    }

    fn bi_get_unsigned_bytes(&self, _reference: Handle) -> BoxedBytes {
        unreachable!()
    }

    fn bi_set_unsigned_bytes(&self, _destination: Handle, _bytes: &[u8]) {
        unreachable!()
    }

    fn bi_signed_byte_length(&self, _x: Handle) -> usize {
        unreachable!()
    }

    fn bi_get_signed_bytes(&self, _reference: Handle) -> BoxedBytes {
        unreachable!()
    }

    fn bi_set_signed_bytes(&self, _destination: Handle, _bytes: &[u8]) {
        unreachable!()
    }

    fn bi_to_i64(&self, _reference: Handle) -> Option<i64> {
        unreachable!()
    }

    fn bi_add(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn bi_sub(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn bi_sub_unsigned(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn bi_mul(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn bi_t_div(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn bi_t_mod(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn bi_abs(&self, _dest: Handle, _x: Handle) {
        unreachable!()
    }

    fn bi_neg(&self, _dest: Handle, _x: Handle) {
        unreachable!()
    }

    fn bi_sign(&self, _x: Handle) -> Sign {
        unreachable!()
    }

    fn bi_cmp(&self, _x: Handle, _y: Handle) -> Ordering {
        unreachable!()
    }

    fn bi_sqrt(&self, _dest: Handle, _x: Handle) {
        unreachable!()
    }

    fn bi_pow(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn bi_log2(&self, _x: Handle) -> u32 {
        unreachable!()
    }

    fn bi_and(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn bi_or(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn bi_xor(&self, _dest: Handle, _x: Handle, _y: Handle) {
        unreachable!()
    }

    fn bi_shr(&self, _dest: Handle, _x: Handle, _bits: usize) {
        unreachable!()
    }

    fn bi_shl(&self, _dest: Handle, _x: Handle, _bits: usize) {
        unreachable!()
    }
}
