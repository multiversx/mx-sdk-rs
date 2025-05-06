use crate::{
    host::vm_hooks::{VMHooksHandlerSource, VMHooksSignalError},
    types::RawHandle,
    vm_err_msg,
};
use core::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Neg, Sub},
};
use multiversx_chain_vm_executor::VMHooksError;
use num_bigint::BigInt;
use num_traits::{ToPrimitive, Zero};
use std::convert::TryInto;

macro_rules! binary_op_method {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(
            &mut self,
            dest: RawHandle,
            x: RawHandle,
            y: RawHandle,
        ) -> Result<(), VMHooksError> {
            self.use_gas(2 * self.gas_schedule().big_float_api_cost.big_float_get_const)?;
            //TODO: check if it's bigint or i64
            self.use_gas(self.gas_schedule().big_float_api_cost.big_float_set_big_int)?;
            // TODO: add big_float_ to rust_op_name
            // self.use_gas(self.gas_schedule().big_float_api_cost.big_float_$rust_op_name)?;

            let bf_x = self.m_types_lock().bf_get_f64(x);
            let bf_y = self.m_types_lock().bf_get_f64(y);
            let result = bf_x.$rust_op_name(bf_y);
            self.m_types_lock().bf_overwrite(dest, result);

            Ok(())
        }
    };
}

macro_rules! unary_op_method {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(&mut self, dest: RawHandle, x: RawHandle) -> Result<(), VMHooksError> {
            self.use_gas(self.gas_schedule().big_float_api_cost.big_float_get_const)?;
            self.use_gas(self.gas_schedule().big_float_api_cost.big_float_set_big_int)?;
            // TODO: add big_float_ to rust_op_name
            // self.use_gas(self.gas_schedule().big_float_api_cost.big_float_$rust_op_name)?;

            let bf_x = self.m_types_lock().bf_get_f64(x);
            let result = bf_x.$rust_op_name();
            self.m_types_lock().bf_overwrite(dest, result);

            Ok(())
        }
    };
}
macro_rules! unary_op_method_big_int_handle {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(&mut self, dest: RawHandle, x: RawHandle) -> Result<(), VMHooksError> {
            self.use_gas(self.gas_schedule().big_float_api_cost.big_float_get_const)?;
            self.use_gas(self.gas_schedule().big_float_api_cost.big_float_set_big_int)?;
            // TODO: add big_float_ to rust_op_name
            // self.use_gas(self.gas_schedule().big_float_api_cost.big_float_$rust_op_name)?;

            let bf_x = self.m_types_lock().bf_get_f64(x);
            let result = bf_x.$rust_op_name();
            self.m_types_lock()
                .bi_overwrite(dest, BigInt::from(result as i64));

            Ok(())
        }
    };
}

pub trait VMHooksBigFloat: VMHooksHandlerSource + VMHooksSignalError {
    fn bf_from_parts(
        &mut self,
        integral_part: i32,
        fractional_part: i32,
        exponent: i32,
    ) -> RawHandle {
        if exponent > 0 {
            self.vm_error_legacy(vm_err_msg::EXPONENT_IS_POSITIVE);
        }

        let exponent_multiplier = (10.0_f64).powi(exponent);
        let fractional_part = f64::from(fractional_part) * exponent_multiplier;
        let mut value = f64::from(integral_part);
        if value > 0f64 {
            value += fractional_part;
        } else {
            value -= fractional_part;
        }

        let mut managed_types = self.m_types_lock();
        managed_types.big_float_map.insert_new_handle_raw(value)
    }

    fn bf_from_frac(&mut self, numerator: i64, denominator: i64) -> RawHandle {
        if denominator == 0 {
            self.vm_error_legacy(vm_err_msg::DIVISION_BY_0);
        }
        let value = if let (Some(f_numerator), Some(f_denominator)) =
            (numerator.to_f64(), denominator.to_f64())
        {
            f_numerator / f_denominator
        } else {
            f64::from(0)
        };

        let mut managed_types = self.m_types_lock();
        managed_types.big_float_map.insert_new_handle_raw(value)
    }

    fn bf_from_sci(&mut self, significand: i64, exponent: i64) -> RawHandle {
        if exponent > 0 {
            self.vm_error_legacy(vm_err_msg::EXPONENT_IS_POSITIVE);
        }

        let value = if let Some(f_significand) = significand.to_f64() {
            let exponent_multiplier = (10.0_f64).powi(exponent.try_into().unwrap());
            f_significand * exponent_multiplier
        } else {
            f64::from(0)
        };

        let mut managed_types = self.m_types_lock();
        managed_types.big_float_map.insert_new_handle_raw(value)
    }

    binary_op_method!(bf_add, add);
    binary_op_method!(bf_sub, sub);
    binary_op_method!(bf_mul, mul);
    binary_op_method!(bf_div, div);

    unary_op_method!(bf_abs, abs);
    unary_op_method!(bf_neg, neg);

    fn bf_cmp(&mut self, x: RawHandle, y: RawHandle) -> i32 {
        let bf_x = self.m_types_lock().bf_get_f64(x);
        let bf_y = self.m_types_lock().bf_get_f64(y);
        let order_opt = bf_x.partial_cmp(&bf_y);
        match order_opt {
            Some(Ordering::Less) => -1,
            Some(Ordering::Equal) => 0,
            Some(Ordering::Greater) => 1,
            None => {
                self.vm_error_legacy(vm_err_msg::CANNOT_COMPARE_VALUES);
                -2
            },
        }
    }

    fn bf_sign(&mut self, x: RawHandle) -> i32 {
        let bf = self.m_types_lock().bf_get_f64(x);
        if bf.is_zero() {
            return 0;
        }

        if !bf.is_normal() {
            self.vm_error_legacy(vm_err_msg::NUMBER_IS_NOT_NORMAL)
        }

        if bf.is_sign_positive() {
            1
        } else {
            -1
        }
    }

    fn bf_clone(&mut self, dest: RawHandle, x: RawHandle) -> Result<(), VMHooksError> {
        self.use_gas(self.gas_schedule().big_float_api_cost.big_float_get_const)?;
        //TODO: check if it's bigint or i64
        self.use_gas(self.gas_schedule().big_float_api_cost.big_float_set_int_64)?;

        let value = self.m_types_lock().bf_get_f64(x);
        self.m_types_lock().bf_overwrite(dest, value);

        Ok(())
    }

    fn bf_sqrt(&mut self, dest: RawHandle, x: RawHandle) -> Result<(), VMHooksError> {
        self.use_gas(self.gas_schedule().big_float_api_cost.big_float_get_const)?;
        //TODO: check if it's i64 or bigint
        self.use_gas(self.gas_schedule().big_float_api_cost.big_float_set_int_64)?;

        let bf_x = self.m_types_lock().bf_get_f64(x);
        if bf_x < 0f64 {
            return self.vm_error(vm_err_msg::BAD_BOUNDS_LOWER);
        }
        let result = bf_x.sqrt();
        self.m_types_lock().bf_overwrite(dest, result);

        Ok(())
    }

    fn bf_pow(&mut self, dest: RawHandle, x: RawHandle, exp: i32) -> Result<(), VMHooksError> {
        self.use_gas(self.gas_schedule().big_float_api_cost.big_float_get_const)?;
        //TODO: check if it's i64 or bigint
        self.use_gas(self.gas_schedule().big_float_api_cost.big_float_set_big_int)?;

        let value = self.m_types_lock().bf_get_f64(x);
        self.m_types_lock().bf_overwrite(dest, value.powi(exp));

        Ok(())
    }

    unary_op_method_big_int_handle!(bf_floor, floor);
    unary_op_method_big_int_handle!(bf_ceil, ceil);
    unary_op_method_big_int_handle!(bf_trunc, trunc);

    fn bf_is_bi(&self, x: RawHandle) -> bool {
        let bf_x = self.m_types_lock().bf_get_f64(x);
        let trunc_x = bf_x.trunc();
        let float_trunc_x = trunc_x.to_f64().unwrap();
        bf_x == float_trunc_x
    }

    fn bf_set_i64(&mut self, dest: RawHandle, value: i64) -> Result<(), VMHooksError> {
        self.use_gas(self.gas_schedule().big_float_api_cost.big_float_set_int_64)?;

        let f64_value = value.to_f64().unwrap();
        self.m_types_lock().bf_overwrite(dest, f64_value);

        Ok(())
    }

    fn bf_set_bi(&mut self, dest: RawHandle, bi: RawHandle) -> Result<(), VMHooksError> {
        self.use_gas(self.gas_schedule().big_float_api_cost.big_float_set_big_int)?;

        let f64_value = self.m_types_lock().bi_to_i64(bi).unwrap().to_f64().unwrap();
        self.m_types_lock().bf_overwrite(dest, f64_value);

        Ok(())
    }

    fn bf_get_const_pi(&mut self, dest: RawHandle) -> Result<(), VMHooksError> {
        self.use_gas(self.gas_schedule().big_float_api_cost.big_float_set_int_64)?;

        self.m_types_lock().bf_overwrite(dest, std::f64::consts::PI);

        Ok(())
    }

    fn bf_get_const_e(&mut self, dest: RawHandle) -> Result<(), VMHooksError> {
        self.use_gas(self.gas_schedule().big_float_api_cost.big_float_set_int_64)?;

        self.m_types_lock().bf_overwrite(dest, std::f64::consts::E);

        Ok(())
    }
}
