use core::cmp::Ordering;

use crate::types::BoxedBytes;

use super::Handle;

/// Only used for sending sign information from the API.
pub enum Sign {
    Minus,
    NoSign,
    Plus,
}

/// Definition of the BigInt type required by the API.
pub trait BigIntApi {
    fn bi_new(&self, value: i64) -> Handle;

    fn bi_new_zero(&self) -> Handle {
        self.bi_new(0)
    }

    fn bi_unsigned_byte_length(&self, handle: Handle) -> usize;
    fn bi_get_unsigned_bytes(&self, handle: Handle) -> BoxedBytes;
    fn bi_set_unsigned_bytes(&self, destination: Handle, bytes: &[u8]);

    fn bi_signed_byte_length(&self, handle: Handle) -> usize;
    fn bi_get_signed_bytes(&self, handle: Handle) -> BoxedBytes;
    fn bi_set_signed_bytes(&self, destination: Handle, bytes: &[u8]);

    fn bi_to_i64(&self, handle: Handle) -> Option<i64>;

    fn bi_add(&self, dest: Handle, x: Handle, y: Handle);
    fn bi_sub(&self, dest: Handle, x: Handle, y: Handle);
    fn bi_sub_unsigned(&self, dest: Handle, x: Handle, y: Handle);
    fn bi_mul(&self, dest: Handle, x: Handle, y: Handle);
    fn bi_t_div(&self, dest: Handle, x: Handle, y: Handle);
    fn bi_t_mod(&self, dest: Handle, x: Handle, y: Handle);

    fn bi_abs(&self, dest: Handle, x: Handle);
    fn bi_neg(&self, dest: Handle, x: Handle);
    fn bi_sign(&self, x: Handle) -> Sign;
    fn bi_cmp(&self, x: Handle, y: Handle) -> Ordering;

    fn bi_sqrt(&self, dest: Handle, x: Handle);
    fn bi_pow(&self, dest: Handle, x: Handle, y: Handle);
    fn bi_log2(&self, x: Handle) -> u32;

    fn bi_and(&self, dest: Handle, x: Handle, y: Handle);
    fn bi_or(&self, dest: Handle, x: Handle, y: Handle);
    fn bi_xor(&self, dest: Handle, x: Handle, y: Handle);
    fn bi_shr(&self, dest: Handle, x: Handle, bits: usize);
    fn bi_shl(&self, dest: Handle, x: Handle, bits: usize);
}
