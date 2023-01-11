use core::cmp::Ordering;

use crate::{api::unsafe_buffer, error_hook};

use multiversx_sc::{
    api::{BigIntApi, Sign},
    err_msg,
    types::heap::BoxedBytes,
};

extern "C" {
    fn bigIntNew(value: i64) -> i32;

    fn bigIntSetInt64(destination: i32, value: i64);
    fn bigIntUnsignedByteLength(x: i32) -> i32;
    fn bigIntGetUnsignedBytes(reference: i32, byte_ptr: *mut u8) -> i32;
    fn bigIntSetUnsignedBytes(destination: i32, byte_ptr: *const u8, byte_len: i32);

    fn bigIntSignedByteLength(x: i32) -> i32;
    fn bigIntGetSignedBytes(reference: i32, byte_ptr: *mut u8) -> i32;
    fn bigIntSetSignedBytes(destination: i32, byte_ptr: *const u8, byte_len: i32);

    fn bigIntIsInt64(reference: i32) -> i32;
    fn bigIntGetInt64(reference: i32) -> i64;

    fn bigIntAdd(dest: i32, x: i32, y: i32);
    fn bigIntSub(dest: i32, x: i32, y: i32);
    fn bigIntMul(dest: i32, x: i32, y: i32);
    fn bigIntTDiv(dest: i32, x: i32, y: i32);
    fn bigIntTMod(dest: i32, x: i32, y: i32);

    fn bigIntAbs(dest: i32, x: i32);
    fn bigIntNeg(dest: i32, x: i32);
    fn bigIntSign(x: i32) -> i32;
    fn bigIntCmp(x: i32, y: i32) -> i32;

    fn bigIntSqrt(dest: i32, x: i32);
    fn bigIntPow(dest: i32, x: i32, y: i32);
    fn bigIntLog2(x: i32) -> i32;

    fn bigIntAnd(dest: i32, x: i32, y: i32);
    fn bigIntOr(dest: i32, x: i32, y: i32);
    fn bigIntXor(dest: i32, x: i32, y: i32);
    fn bigIntShr(dest: i32, x: i32, bits: i32);
    fn bigIntShl(dest: i32, x: i32, bits: i32);

    fn bigIntToString(bigIntHandle: i32, destHandle: i32);
}

macro_rules! binary_op_wrapper {
    ($method_name:ident, $hook_name:ident) => {
        fn $method_name(
            &self,
            dest: Self::BigIntHandle,
            x: Self::BigIntHandle,
            y: Self::BigIntHandle,
        ) {
            unsafe {
                $hook_name(dest, x, y);
            }
        }
    };
}

macro_rules! unary_op_wrapper {
    ($method_name:ident, $hook_name:ident) => {
        fn $method_name(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle) {
            unsafe {
                $hook_name(dest, x);
            }
        }
    };
}

impl BigIntApi for crate::api::VmApiImpl {
    #[inline]
    fn bi_new(&self, value: i64) -> Self::BigIntHandle {
        unsafe { bigIntNew(value) }
    }

    #[inline]
    fn bi_set_int64(&self, destination: Self::BigIntHandle, value: i64) {
        unsafe {
            bigIntSetInt64(destination, value);
        }
    }

    #[inline]
    fn bi_unsigned_byte_length(&self, x: Self::BigIntHandle) -> usize {
        unsafe { bigIntUnsignedByteLength(x) as usize }
    }

    fn bi_get_unsigned_bytes(&self, handle: Self::BigIntHandle) -> BoxedBytes {
        unsafe {
            let byte_len = bigIntUnsignedByteLength(handle);
            let mut bb = BoxedBytes::allocate(byte_len as usize);
            bigIntGetUnsignedBytes(handle, bb.as_mut_ptr());
            bb
        }
    }

    #[inline]
    fn bi_set_unsigned_bytes(&self, destination: Self::BigIntHandle, bytes: &[u8]) {
        unsafe { bigIntSetUnsignedBytes(destination, bytes.as_ptr(), bytes.len() as i32) }
    }

    #[inline]
    fn bi_signed_byte_length(&self, x: Self::BigIntHandle) -> usize {
        unsafe { bigIntSignedByteLength(x) as usize }
    }

    fn bi_get_signed_bytes(&self, handle: Self::BigIntHandle) -> BoxedBytes {
        unsafe {
            let byte_len = bigIntSignedByteLength(handle);
            let mut bb = BoxedBytes::allocate(byte_len as usize);
            bigIntGetSignedBytes(handle, bb.as_mut_ptr());
            bb
        }
    }

    #[inline]
    fn bi_set_signed_bytes(&self, destination: Self::BigIntHandle, bytes: &[u8]) {
        unsafe { bigIntSetSignedBytes(destination, bytes.as_ptr(), bytes.len() as i32) }
    }

    fn bi_to_i64(&self, reference: Self::BigIntHandle) -> Option<i64> {
        unsafe {
            let is_i64_result = bigIntIsInt64(reference);
            if is_i64_result > 0 {
                Some(bigIntGetInt64(reference))
            } else {
                None
            }
        }
    }

    binary_op_wrapper! {bi_add, bigIntAdd}
    binary_op_wrapper! {bi_sub, bigIntSub}

    fn bi_sub_unsigned(
        &self,
        dest: Self::BigIntHandle,
        x: Self::BigIntHandle,
        y: Self::BigIntHandle,
    ) {
        unsafe {
            bigIntSub(dest, x, y);
            if bigIntSign(dest) < 0 {
                error_hook::signal_error(err_msg::BIG_UINT_SUB_NEGATIVE)
            }
        }
    }

    binary_op_wrapper! {bi_mul, bigIntMul}
    binary_op_wrapper! {bi_t_div, bigIntTDiv}
    binary_op_wrapper! {bi_t_mod, bigIntTMod}

    unary_op_wrapper! {bi_abs, bigIntAbs}
    unary_op_wrapper! {bi_neg, bigIntNeg}

    fn bi_sign(&self, x: Self::BigIntHandle) -> Sign {
        unsafe {
            match bigIntSign(x).cmp(&0) {
                Ordering::Greater => Sign::Plus,
                Ordering::Equal => Sign::NoSign,
                Ordering::Less => Sign::Minus,
            }
        }
    }

    #[inline]
    fn bi_cmp(&self, x: Self::BigIntHandle, y: Self::BigIntHandle) -> Ordering {
        unsafe { bigIntCmp(x, y).cmp(&0) }
    }

    unary_op_wrapper! {bi_sqrt, bigIntSqrt}
    binary_op_wrapper! {bi_pow, bigIntPow}

    fn bi_log2(&self, x: Self::BigIntHandle) -> u32 {
        unsafe { bigIntLog2(x) as u32 }
    }

    binary_op_wrapper! {bi_and, bigIntAnd}
    binary_op_wrapper! {bi_or, bigIntOr}
    binary_op_wrapper! {bi_xor, bigIntXor}

    fn bi_shr(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, bits: usize) {
        unsafe {
            bigIntShr(dest, x, bits as i32);
        }
    }

    fn bi_shl(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, bits: usize) {
        unsafe {
            bigIntShl(dest, x, bits as i32);
        }
    }

    fn bi_to_string(
        &self,
        bi_handle: Self::BigIntHandle,
        result_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            bigIntToString(bi_handle, result_handle);
        }
    }
}

#[allow(unused)]
pub(crate) unsafe fn unsafe_buffer_load_be_pad_right(bi_handle: i32, nr_bytes: usize) -> *const u8 {
    let byte_len = bigIntUnsignedByteLength(bi_handle) as usize;
    if byte_len > nr_bytes {
        error_hook::signal_error(err_msg::BIG_UINT_EXCEEDS_SLICE);
    }
    unsafe_buffer::clear_buffer_1();
    if byte_len > 0 {
        bigIntGetUnsignedBytes(
            bi_handle,
            unsafe_buffer::buffer_1_ptr().add(nr_bytes - byte_len),
        );
    }
    unsafe_buffer::buffer_1_ptr()
}
