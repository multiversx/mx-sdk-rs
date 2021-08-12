use core::cmp::Ordering;

use elrond_wasm::api::{BigIntApi, Handle, Sign};
use elrond_wasm::types::BoxedBytes;

extern "C" {
    fn bigIntNew(value: i64) -> i32;

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

    fn bigIntPow(dest: i32, x: i32, y: i32);
    fn bigIntAbs(dest: i32, x: i32);
    fn bigIntNeg(dest: i32, x: i32);
    fn bigIntSign(x: i32) -> i32;
    fn bigIntCmp(x: i32, y: i32) -> i32;
}

macro_rules! binary_op_wrapper {
    ($method_name:ident, $hook_name:ident) => {
        fn $method_name(&self, dest: Handle, x: Handle, y: Handle) {
            unsafe {
                $hook_name(dest, x, y);
            }
        }
    };
}

impl BigIntApi for crate::ArwenApiImpl {
    fn new(&self, value: i64) -> Handle {
        unsafe { bigIntNew(value) }
    }

    fn signed_byte_length(&self, x: Handle) -> Handle {
        unsafe { bigIntSignedByteLength(x) }
    }

    fn get_signed_bytes(&self, handle: Handle) -> BoxedBytes {
        unsafe {
            let byte_len = bigIntSignedByteLength(handle);
            let mut bb = BoxedBytes::allocate(byte_len as usize);
            bigIntGetSignedBytes(handle, bb.as_mut_ptr());
            bb
        }
    }

    fn set_signed_bytes(&self, destination: Handle, bytes: &[u8]) {
        unsafe { bigIntSetSignedBytes(destination, bytes.as_ptr(), bytes.len() as i32) }
    }

    fn bi_to_i64(&self, reference: Handle) -> Option<i64> {
        unsafe {
            let is_i64_result = bigIntIsInt64(reference);
            if is_i64_result > 0 {
                Some(bigIntGetInt64(reference))
            } else {
                None
            }
        }
    }

    binary_op_wrapper! {add, bigIntAdd}
    binary_op_wrapper! {sub, bigIntSub}
    binary_op_wrapper! {mul, bigIntMul}
    binary_op_wrapper! {t_div, bigIntTDiv}
    binary_op_wrapper! {t_mod, bigIntTMod}
    binary_op_wrapper! {pow, bigIntPow}

    fn abs(&self, dest: Handle, x: Handle) {
        unsafe {
            bigIntAbs(dest, x);
        }
    }

    fn neg(&self, dest: Handle, x: Handle) {
        unsafe {
            bigIntNeg(dest, x);
        }
    }

    fn sign(&self, x: Handle) -> Sign {
        unsafe {
            match bigIntSign(x).cmp(&0) {
                Ordering::Greater => Sign::Plus,
                Ordering::Equal => Sign::NoSign,
                Ordering::Less => Sign::Minus,
            }
        }
    }

    fn cmp(&self, x: Handle, y: Handle) -> Ordering {
        unsafe { bigIntCmp(x, y).cmp(&0) }
    }
}
