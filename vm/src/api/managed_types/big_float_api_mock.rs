use crate::{tx_mock::TxPanic, DebugApi};
use core::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Neg, Sub},
};
use std::convert::TryInto;

use multiversx_sc::{
    api::{BigFloatApi, BigIntApi, ErrorApiImpl, HandleTypeInfo, Sign},
    codec::num_bigint::BigInt,
    err_msg,
};
use num_traits::ToPrimitive;

impl DebugApi {
    pub(crate) fn bf_get_f64(&self, handle: <Self as HandleTypeInfo>::BigFloatHandle) -> f64 {
        let managed_types = handle.context.m_types_borrow_mut();
        *managed_types
            .big_float_map
            .get(handle.get_raw_handle_unchecked())
    }

    pub(crate) fn bf_overwrite(
        &self,
        handle: <Self as HandleTypeInfo>::BigFloatHandle,
        value: f64,
    ) {
        let mut managed_types = handle.context.m_types_borrow_mut();
        managed_types
            .big_float_map
            .insert(handle.get_raw_handle_unchecked(), value);
    }
}

macro_rules! binary_op_method {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(
            &self,
            dest: Self::BigFloatHandle,
            x: Self::BigFloatHandle,
            y: Self::BigFloatHandle,
        ) {
            let bf_x = self.bf_get_f64(x);
            let bf_y = self.bf_get_f64(y);
            let result = bf_x.$rust_op_name(bf_y);
            self.bf_overwrite(dest, result);
        }
    };
}

macro_rules! unary_op_method {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(&self, dest: Self::BigFloatHandle, x: Self::BigFloatHandle) {
            let bf_x = self.bf_get_f64(x);
            let result = bf_x.$rust_op_name();
            self.bf_overwrite(dest, result);
        }
    };
}
macro_rules! unary_op_method_big_int_handle {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(&self, dest: Self::BigIntHandle, x: Self::BigFloatHandle) {
            let bf_x = self.bf_get_f64(x);
            let result = bf_x.$rust_op_name();
            self.bi_overwrite(dest, BigInt::from(result as i64));
        }
    };
}

impl BigFloatApi for DebugApi {
    fn bf_from_parts(
        &self,
        integral_part: i32,
        fractional_part: i32,
        exponent: i32,
    ) -> Self::ManagedBufferHandle {
        if exponent > 0 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::EXPONENT_IS_POSITIVE.to_string(),
            });
        }

        let exponent_multiplier = (10.0_f64).powi(exponent);
        let fractional_part = f64::from(fractional_part) * exponent_multiplier;
        let mut value = f64::from(integral_part);
        if value > 0f64 {
            value += fractional_part;
        } else {
            value -= fractional_part;
        }

        let mut managed_types = self.m_types_borrow_mut();
        managed_types.big_float_map.insert_new_handle(value)
    }

    fn bf_from_frac(&self, numerator: i64, denominator: i64) -> Self::BigFloatHandle {
        if denominator == 0 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::DIVISION_BY_0.to_string(),
            });
        }
        let value = if let (Some(f_numerator), Some(f_denominator)) =
            (numerator.to_f64(), denominator.to_f64())
        {
            f_numerator / f_denominator
        } else {
            f64::from(0)
        };

        let mut managed_types = self.m_types_borrow_mut();
        managed_types.big_float_map.insert_new_handle(value)
    }

    fn bf_from_sci(&self, significand: i64, exponent: i64) -> Self::BigFloatHandle {
        if exponent > 0 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::EXPONENT_IS_POSITIVE.to_string(),
            });
        }

        let value = if let Some(f_significand) = significand.to_f64() {
            let exponent_multiplier = (10.0_f64).powi(exponent.try_into().unwrap());
            f_significand * exponent_multiplier
        } else {
            f64::from(0)
        };

        let mut managed_types = self.m_types_borrow_mut();
        managed_types.big_float_map.insert_new_handle(value)
    }

    binary_op_method!(bf_add, add);
    binary_op_method!(bf_sub, sub);
    binary_op_method!(bf_mul, mul);
    binary_op_method!(bf_div, div);

    unary_op_method!(bf_abs, abs);
    unary_op_method!(bf_neg, neg);
    fn bf_cmp(&self, x: Self::BigFloatHandle, y: Self::BigFloatHandle) -> Ordering {
        let bf_x = self.bf_get_f64(x);
        let bf_y = self.bf_get_f64(y);
        let order_opt = bf_x.partial_cmp(&bf_y);
        if order_opt.is_none() {
            self.signal_error(err_msg::CANNOT_COMPARE_VALUES)
        }
        order_opt.unwrap()
    }

    fn bf_sign(&self, x: Self::BigFloatHandle) -> Sign {
        let bf = self.bf_get_f64(x);
        if !bf.is_normal() {
            self.signal_error(err_msg::NUMBER_IS_NOT_NORMAL)
        }

        if bf.is_sign_positive() {
            return multiversx_sc::api::Sign::Plus;
        } else if bf.is_sign_negative() {
            return multiversx_sc::api::Sign::Minus;
        }
        multiversx_sc::api::Sign::NoSign
    }

    fn bf_clone(&self, dest: Self::BigFloatHandle, x: Self::BigFloatHandle) {
        let value = self.bf_get_f64(x);
        self.bf_overwrite(dest, value);
    }

    fn bf_sqrt(&self, dest: Self::BigFloatHandle, x: Self::BigFloatHandle) {
        let bf_x = self.bf_get_f64(x);
        if bf_x < 0f64 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::BAD_BOUNDS_LOWER.to_string(),
            });
        }
        let result = bf_x.sqrt();
        self.bf_overwrite(dest, result);
    }

    fn bf_pow(&self, dest: Self::BigFloatHandle, x: Self::BigFloatHandle, exp: i32) {
        let value = self.bf_get_f64(x);
        self.bf_overwrite(dest, value.powi(exp));
    }

    unary_op_method_big_int_handle!(bf_floor, floor);
    unary_op_method_big_int_handle!(bf_ceil, ceil);
    unary_op_method_big_int_handle!(bf_trunc, trunc);

    fn bf_is_bi(&self, x: Self::BigFloatHandle) -> bool {
        let bf_x = self.bf_get_f64(x);
        let trunc_x = bf_x.trunc();
        let float_trunc_x = trunc_x.to_f64().unwrap();
        bf_x == float_trunc_x
    }

    fn bf_set_i64(&self, dest: Self::BigFloatHandle, value: i64) {
        let f64_value = value.to_f64().unwrap();
        self.bf_overwrite(dest, f64_value);
    }

    fn bf_set_bi(&self, dest: Self::BigFloatHandle, bi: Self::BigIntHandle) {
        let f64_value = self.bi_to_i64(bi).unwrap().to_f64().unwrap();
        self.bf_overwrite(dest, f64_value);
    }

    fn bf_get_const_pi(&self, dest: Self::BigFloatHandle) {
        self.bf_overwrite(dest, std::f64::consts::PI);
    }

    fn bf_get_const_e(&self, dest: Self::BigFloatHandle) {
        self.bf_overwrite(dest, std::f64::consts::E);
    }
}
