use core::cmp::Ordering;

use crate::{
    api::{ErrorApi, ErrorApiImpl},
    err_msg,
};

use super::HandleTypeInfo;

/// Only used for sending sign information from the API.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Minus,
    NoSign,
    Plus,
}

/// Definition of the BigInt type required by the API.
pub trait BigIntApiImpl: HandleTypeInfo + ErrorApi {
    fn bi_new(&self, value: i64) -> Self::BigIntHandle;

    fn bi_new_zero(&self) -> Self::BigIntHandle {
        self.bi_new(0)
    }

    fn bi_set_int64(&self, destination: Self::BigIntHandle, value: i64);
    fn bi_to_i64(&self, handle: Self::BigIntHandle) -> Option<i64>;

    fn bi_add(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, y: Self::BigIntHandle);
    fn bi_sub(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, y: Self::BigIntHandle);

    fn bi_sub_unsigned(
        &self,
        dest: Self::BigIntHandle,
        x: Self::BigIntHandle,
        y: Self::BigIntHandle,
    ) {
        self.bi_sub(dest.clone(), x, y);
        if self.bi_sign(dest) == Sign::Minus {
            Self::error_api_impl().signal_error(err_msg::BIG_UINT_SUB_NEGATIVE);
        }
    }

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
