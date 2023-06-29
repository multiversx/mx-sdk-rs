use core::cmp::Ordering;

use multiversx_sc::api::{BigIntApiImpl, Sign};

extern "C" {
    fn bigIntNew(value: i64) -> i32;

    fn bigIntSetInt64(destination: i32, value: i64);
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

impl BigIntApiImpl for crate::api::VmApiImpl {
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
