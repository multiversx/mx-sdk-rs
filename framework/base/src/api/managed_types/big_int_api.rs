use core::cmp::Ordering;

use crate::types::heap::BoxedBytes;

use super::HandleTypeInfo;

/// Only used for sending sign information from the API.
pub enum Sign {
    Minus,
    NoSign,
    Plus,
}

/// Definition of the BigInt type required by the API.
pub trait BigIntApi: HandleTypeInfo {
    fn bi_new(&self, value: i64) -> Self::BigIntHandle;

    fn bi_new_zero(&self) -> Self::BigIntHandle {
        self.bi_new(0)
    }

    fn bi_set_int64(&self, destination: Self::BigIntHandle, value: i64);
    fn bi_unsigned_byte_length(&self, handle: Self::BigIntHandle) -> usize;
    fn bi_get_unsigned_bytes(&self, handle: Self::BigIntHandle) -> BoxedBytes;
    fn bi_set_unsigned_bytes(&self, destination: Self::BigIntHandle, bytes: &[u8]);

    fn bi_signed_byte_length(&self, handle: Self::BigIntHandle) -> usize;
    fn bi_get_signed_bytes(&self, handle: Self::BigIntHandle) -> BoxedBytes;
    fn bi_set_signed_bytes(&self, destination: Self::BigIntHandle, bytes: &[u8]);

    fn bi_to_i64(&self, handle: Self::BigIntHandle) -> Option<i64>;

    fn bi_add(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, y: Self::BigIntHandle);
    fn bi_sub(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, y: Self::BigIntHandle);
    fn bi_sub_unsigned(
        &self,
        dest: Self::BigIntHandle,
        x: Self::BigIntHandle,
        y: Self::BigIntHandle,
    );
    fn bi_mul(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, y: Self::BigIntHandle);
    fn bi_t_div(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, y: Self::BigIntHandle);
    fn bi_t_mod(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, y: Self::BigIntHandle);

    fn bi_abs(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle);
    fn bi_neg(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle);
    fn bi_sign(&self, x: Self::BigIntHandle) -> Sign;
    fn bi_cmp(&self, x: Self::BigIntHandle, y: Self::BigIntHandle) -> Ordering;

    fn bi_sqrt(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle);
    fn bi_pow(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, y: Self::BigIntHandle);
    fn bi_log2(&self, x: Self::BigIntHandle) -> u32;

    fn bi_and(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, y: Self::BigIntHandle);
    fn bi_or(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, y: Self::BigIntHandle);
    fn bi_xor(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, y: Self::BigIntHandle);
    fn bi_shr(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, bits: usize);
    fn bi_shl(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, bits: usize);

    fn bi_to_string(&self, bi_handle: Self::BigIntHandle, str_handle: Self::ManagedBufferHandle);
}
