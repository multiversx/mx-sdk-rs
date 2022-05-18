use crate::DebugApi;
use core::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Neg, Sub},
};

use elrond_wasm::{
    api::{BigFloatApi, BigIntApi, ErrorApiImpl, Handle, Sign},
    err_msg,
};
use num_traits::{Signed, ToPrimitive};

impl DebugApi {
    pub(crate) fn bf_get_f64(&self, handle: Handle) -> f64 {
        let managed_types = self.m_types_borrow_mut();
        *managed_types.big_float_map.get(handle)
    }

    pub(crate) fn bf_overwrite(&self, handle: Handle, value: f64) {
        let mut managed_types = self.m_types_borrow_mut();
        managed_types.big_float_map.insert(handle, value);
    }
}

macro_rules! binary_op_method {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(&self, dest: Handle, x: Handle, y: Handle) {
            let bf_x = self.bf_get_f64(x);
            let bf_y = self.bf_get_f64(y);
            let result = bf_x.$rust_op_name(bf_y);
            self.bf_overwrite(dest, result);
        }
    };
}

macro_rules! unary_op_method {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(&self, dest: Handle, x: Handle) {
            let bf_x = self.bf_get_f64(x);
            let result = bf_x.$rust_op_name();
            self.bf_overwrite(dest, result);
        }
    };
}

impl BigFloatApi for DebugApi {
    fn bf_from_parts(&self, integral_part: i32, fractional_part: i32, exponent: i32) -> Handle {
        if exponent > 0 {
            self.signal_error(err_msg::EXPONENT_IS_POSITIVE)
        }

        let exponent_multiplier = (10.0_f64).powi(exponent);
        let fractional_part = f64::from(fractional_part) * exponent_multiplier;
        let value = f64::from(integral_part) + fractional_part;

        let mut managed_types = self.m_types_borrow_mut();
        managed_types.big_float_map.insert_new_handle(value)
    }
    fn bf_from_frac(&self, numerator: i64, denominator: i64) -> Handle {
        let f_numerator = numerator.to_f64();
        let f_denominator = denominator.to_f64();
        let value: f64;

        if f_numerator == None || f_denominator == None {
            value = f64::from(0);
        } else {
            value = f_numerator.unwrap() / f_denominator.unwrap();
        }

        let mut managed_types = self.m_types_borrow_mut();
        managed_types.big_float_map.insert_new_handle(value)
    }

    fn bf_from_sci(&self, significand: i64, exponent: i64) -> Handle {
        if exponent > 0 {
            self.signal_error(err_msg::EXPONENT_IS_POSITIVE)
        }

        let f_significand = significand.to_f64();
        let value: f64;

        if f_significand == None {
            value = f64::from(0);
        } else {
            let exponent_multiplier = (10.0_f64).powi(exponent as i32);
            value = f_significand.unwrap() * exponent_multiplier;
        }

        let mut managed_types = self.m_types_borrow_mut();
        managed_types.big_float_map.insert_new_handle(value)
    }

    binary_op_method!(bf_add, add);
    binary_op_method!(bf_sub, sub);
    binary_op_method!(bf_mul, mul);
    binary_op_method!(bf_div, div);

    unary_op_method!(bf_abs, abs);
    unary_op_method!(bf_neg, neg);
    fn bf_cmp(&self, x: Handle, y: Handle) -> Ordering {
        let managed_types = self.m_types_borrow_mut();
        let bf_x = managed_types.big_float_map.get(x);
        let bf_y = managed_types.big_float_map.get(y);
        let order_opt = bf_x.partial_cmp(bf_y);
        if order_opt == None {
            self.signal_error(err_msg::CANNOT_COMPARE_VALUES)
        }
        order_opt.unwrap()
    }

    fn bf_sign(&self, x: Handle) -> Sign {
        let managed_types = self.m_types_borrow();
        let bf = managed_types.big_float_map.get(x);
        if !bf.is_normal() {
            self.signal_error(err_msg::NUMBER_IS_NOT_NORMAL)
        }

        if bf.is_positive() {
            return elrond_wasm::api::Sign::Plus;
        } else if bf.is_negative() {
            return elrond_wasm::api::Sign::Minus;
        }
        elrond_wasm::api::Sign::NoSign
    }

    fn bf_clone(&self, dest: Handle, x: Handle) {
        let value = self.bf_get_f64(x);
        self.bf_overwrite(dest, value);
    }

    unary_op_method!(bf_sqrt, sqrt);

    fn bf_pow(&self, dest: Handle, x: Handle, exp: i32) {
        let value = self.bf_get_f64(x);
        self.bf_overwrite(dest, value.powi(exp));
    }

    unary_op_method!(bf_floor, floor);
    unary_op_method!(bf_ceil, ceil);
    unary_op_method!(bf_trunc, trunc);

    fn bf_is_bi(&self, x: Handle) -> bool {
        let managed_types = self.m_types_borrow();
        let bf_x = managed_types.big_float_map.get(x);
        let trunc_x = bf_x.trunc();
        let float_trunc_x = trunc_x.to_f64().unwrap();
        *bf_x == float_trunc_x
    }

    fn bf_set_i64(&self, dest: Handle, value: i64) {
        let f64_value = value.to_f64().unwrap();
        self.bf_overwrite(dest, f64_value);
    }

    fn bf_set_bi(&self, dest: Handle, bi: Handle) {
        let f64_value = self.bi_to_i64(bi).unwrap().to_f64().unwrap();
        self.bf_overwrite(dest, f64_value);
    }

    fn bf_get_const_pi(&self, dest: Handle) {
        self.bf_overwrite(dest, std::f64::consts::PI);
    }

    fn bf_get_const_e(&self, dest: Handle) {
        self.bf_overwrite(dest, std::f64::consts::E);
    }
}
