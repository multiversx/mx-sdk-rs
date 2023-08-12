use core::cmp::Ordering;

use multiversx_sc::api::{BigFloatApiImpl, Sign};

extern "C" {
    fn bigFloatNewFromParts(integralPart: i32, fractionalPart: i32, exponent: i32) -> i32;
    fn bigFloatNewFromFrac(numerator: i64, denominator: i64) -> i32;
    fn bigFloatNewFromSci(significand: i64, exponent: i64) -> i32;

    fn bigFloatAdd(dest: i32, x: i32, y: i32);
    fn bigFloatSub(dest: i32, x: i32, y: i32);
    fn bigFloatMul(dest: i32, x: i32, y: i32);
    fn bigFloatDiv(dest: i32, x: i32, y: i32);

    fn bigFloatAbs(dest: i32, x: i32);
    fn bigFloatNeg(dest: i32, x: i32);
    fn bigFloatCmp(x: i32, y: i32) -> i32;
    fn bigFloatSign(x: i32) -> i32;
    fn bigFloatClone(dest: i32, x: i32);
    fn bigFloatSqrt(dest: i32, x: i32);
    fn bigFloatPow(dest: i32, x: i32, exponent: i32);

    fn bigFloatFloor(dest: i32, x: i32);
    fn bigFloatCeil(dest: i32, x: i32);
    fn bigFloatTruncate(dest: i32, x: i32);

    fn bigFloatIsInt(x: i32) -> i32;
    fn bigFloatSetInt64(dest: i32, x: i64);
    fn bigFloatSetBigInt(dest: i32, x: i32);

    fn bigFloatGetConstPi(dest: i32);
    fn bigFloatGetConstE(dest: i32);
}

macro_rules! binary_op_wrapper {
    ($method_name:ident, $hook_name:ident) => {
        fn $method_name(
            &self,
            dest: Self::BigFloatHandle,
            x: Self::BigFloatHandle,
            y: Self::BigFloatHandle,
        ) {
            unsafe {
                $hook_name(dest, x, y);
            }
        }
    };
}

macro_rules! unary_op_wrapper {
    ($method_name:ident, $hook_name:ident) => {
        fn $method_name(&self, dest: Self::BigFloatHandle, x: Self::BigFloatHandle) {
            unsafe {
                $hook_name(dest, x);
            }
        }
    };
}

macro_rules! unary_op_method_big_int_handle {
    ($method_name:ident, $hook_name:ident) => {
        fn $method_name(&self, dest: Self::BigIntHandle, x: Self::BigFloatHandle) {
            unsafe {
                $hook_name(dest, x);
            }
        }
    };
}

impl BigFloatApiImpl for crate::api::VmApiImpl {
    #[inline]
    fn bf_from_parts(
        &self,
        integral_part_value: i32,
        fractional_part_value: i32,
        exponent_value: i32,
    ) -> Self::BigFloatHandle {
        unsafe { bigFloatNewFromParts(integral_part_value, fractional_part_value, exponent_value) }
    }

    #[inline]
    fn bf_from_frac(&self, numerator_value: i64, denominator_value: i64) -> Self::BigFloatHandle {
        unsafe { bigFloatNewFromFrac(numerator_value, denominator_value) }
    }

    #[inline]
    fn bf_from_sci(
        &self,
        significand_value: i64,
        exponent_value: i64,
    ) -> Self::ManagedBufferHandle {
        unsafe { bigFloatNewFromSci(significand_value, exponent_value) }
    }

    binary_op_wrapper! {bf_add, bigFloatAdd}
    binary_op_wrapper! {bf_sub, bigFloatSub}
    binary_op_wrapper! {bf_mul, bigFloatMul}
    binary_op_wrapper! {bf_div, bigFloatDiv}

    unary_op_wrapper! {bf_neg, bigFloatNeg}
    unary_op_wrapper! {bf_abs, bigFloatAbs}

    #[inline]
    fn bf_cmp(&self, x: Self::ManagedBufferHandle, y: Self::ManagedBufferHandle) -> Ordering {
        unsafe { bigFloatCmp(x, y).cmp(&0) }
    }

    fn bf_sign(&self, x: Self::ManagedBufferHandle) -> Sign {
        unsafe {
            match bigFloatSign(x).cmp(&0) {
                Ordering::Greater => Sign::Plus,
                Ordering::Equal => Sign::NoSign,
                Ordering::Less => Sign::Minus,
            }
        }
    }

    unary_op_wrapper! {bf_clone, bigFloatClone}
    unary_op_wrapper! {bf_sqrt, bigFloatSqrt}

    fn bf_pow(&self, dest: Self::BigFloatHandle, x: Self::BigFloatHandle, exp: i32) {
        unsafe {
            bigFloatPow(dest, x, exp);
        }
    }

    unary_op_method_big_int_handle! {bf_floor , bigFloatFloor}
    unary_op_method_big_int_handle! {bf_ceil , bigFloatCeil}
    unary_op_method_big_int_handle! {bf_trunc , bigFloatTruncate}

    #[inline]
    fn bf_is_bi(&self, x: Self::BigFloatHandle) -> bool {
        unsafe { 1 == bigFloatIsInt(x) }
    }

    #[inline]
    fn bf_set_i64(&self, dest: Self::BigFloatHandle, value: i64) {
        unsafe {
            bigFloatSetInt64(dest, value);
        }
    }

    #[inline]
    fn bf_set_bi(&self, dest: Self::BigFloatHandle, x: Self::BigIntHandle) {
        unsafe {
            bigFloatSetBigInt(dest, x);
        }
    }

    #[inline]
    fn bf_get_const_e(&self, dest: Self::BigFloatHandle) {
        unsafe { bigFloatGetConstE(dest) }
    }

    #[inline]
    fn bf_get_const_pi(&self, dest: Self::BigFloatHandle) {
        unsafe { bigFloatGetConstPi(dest) }
    }
}
