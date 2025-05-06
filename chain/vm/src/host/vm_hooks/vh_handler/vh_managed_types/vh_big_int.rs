use crate::{
    host::{
        context::big_int_to_i64,
        vm_hooks::{VMHooksHandlerSource, VMHooksSignalError},
    },
    types::RawHandle,
    vm_err_msg,
};
use core::{
    cmp::Ordering,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Shl, Shr, Sub},
};
use multiversx_chain_vm_executor::VMHooksError;
use num_traits::{pow, sign::Signed};
use std::convert::TryInto;

macro_rules! binary_op_method {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(
            &mut self,
            dest: RawHandle,
            x: RawHandle,
            y: RawHandle,
        ) -> Result<(), VMHooksError> {
            self.use_gas(self.gas_schedule().big_int_api_cost.big_int_get_int_64)?;
            self.use_gas(self.gas_schedule().big_int_api_cost.big_int_get_int_64)?;
            self.use_gas(self.gas_schedule().big_int_api_cost.big_int_set_int_64)?;
            // TODO: add big_int_ to rust_op_name
            // self.use_gas(self.gas_schedule().big_int_api_cost.big_int_$rust_op_name)?;

            let bi_x = self.m_types_lock().bi_get(x);
            let bi_y = self.m_types_lock().bi_get(y);
            let result = bi_x.$rust_op_name(bi_y);
            self.m_types_lock().bi_overwrite(dest, result);

            Ok(())
        }
    };
}

macro_rules! binary_bitwise_op_method {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(
            &mut self,
            dest: RawHandle,
            x: RawHandle,
            y: RawHandle,
        ) -> Result<(), VMHooksError> {
            self.use_gas(self.gas_schedule().big_int_api_cost.big_int_get_int_64)?;
            self.use_gas(self.gas_schedule().big_int_api_cost.big_int_set_int_64)?;
            // TODO: add big_int_ to rust_op_name
            // self.use_gas(self.gas_schedule().big_int_api_cost.big_int_$rust_op_name)?;

            let bi_x = self.m_types_lock().bi_get(x);
            if bi_x.sign() == num_bigint::Sign::Minus {
                self.vm_error(vm_err_msg::BIG_INT_BITWISE_OPERATION_NEGATIVE)?;
            }
            let bi_y = self.m_types_lock().bi_get(y);
            if bi_y.sign() == num_bigint::Sign::Minus {
                self.vm_error(vm_err_msg::BIG_INT_BITWISE_OPERATION_NEGATIVE)?;
            }
            let result = bi_x.$rust_op_name(bi_y);
            self.m_types_lock().bi_overwrite(dest, result);

            Ok(())
        }
    };
}

macro_rules! unary_op_method {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(&mut self, dest: RawHandle, x: RawHandle) -> Result<(), VMHooksError> {
            self.use_gas(self.gas_schedule().big_int_api_cost.big_int_get_int_64)?;
            self.use_gas(self.gas_schedule().big_int_api_cost.big_int_set_int_64)?;
            // TODO: add big_int_ to rust_op_name
            // self.use_gas(self.gas_schedule().big_int_api_cost.big_int_$rust_op_name)?;

            let bi_x = self.m_types_lock().bi_get(x);
            let result = bi_x.$rust_op_name();
            self.m_types_lock().bi_overwrite(dest, result);

            Ok(())
        }
    };
}

/// Provides VM hook implementations for methods that deal big ints.
pub trait VMHooksBigInt: VMHooksHandlerSource + VMHooksSignalError {
    fn bi_new(&mut self, value: i64) -> Result<RawHandle, VMHooksError> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_new)?;

        Ok(self
            .m_types_lock()
            .bi_new_from_big_int(num_bigint::BigInt::from(value)))
    }

    fn bi_set_int64(&mut self, destination: RawHandle, value: i64) -> Result<(), VMHooksError> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_set_int_64)?;

        self.m_types_lock()
            .bi_overwrite(destination, num_bigint::BigInt::from(value));
        Ok(())
    }

    fn bi_unsigned_byte_length(&mut self, handle: RawHandle) -> Result<usize, VMHooksError> {
        self.use_gas(
            self.gas_schedule()
                .big_int_api_cost
                .big_int_unsigned_byte_length,
        )?;

        Ok(self.m_types_lock().bi_get_unsigned_bytes(handle).len())
    }

    fn bi_get_unsigned_bytes(&mut self, handle: RawHandle) -> Result<Vec<u8>, VMHooksError> {
        self.use_gas(
            self.gas_schedule()
                .big_int_api_cost
                .big_int_get_unsigned_bytes,
        )?;

        Ok(self.m_types_lock().bi_get_unsigned_bytes(handle))
    }

    fn bi_set_unsigned_bytes(
        &mut self,
        destination: RawHandle,
        bytes: &[u8],
    ) -> Result<(), VMHooksError> {
        self.use_gas(
            self.gas_schedule()
                .big_int_api_cost
                .big_int_set_unsigned_bytes,
        )?;

        self.m_types_lock()
            .bi_set_unsigned_bytes(destination, bytes);

        Ok(())
    }

    fn bi_get_signed_bytes(&mut self, handle: RawHandle) -> Result<Vec<u8>, VMHooksError> {
        self.use_gas(
            self.gas_schedule()
                .big_int_api_cost
                .big_int_get_signed_bytes,
        )?;

        Ok(self.m_types_lock().bi_get_signed_bytes(handle))
    }

    fn bi_set_signed_bytes(
        &mut self,
        destination: RawHandle,
        bytes: &[u8],
    ) -> Result<(), VMHooksError> {
        self.use_gas(
            self.gas_schedule()
                .big_int_api_cost
                .big_int_set_signed_bytes,
        )?;

        self.m_types_lock().bi_set_signed_bytes(destination, bytes);

        Ok(())
    }

    // TODO: check implications of change
    fn bi_is_int64(&mut self, destination_handle: RawHandle) -> Result<i32, VMHooksError> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_is_int_64)?;

        if self.m_types_lock().bi_to_i64(destination_handle).is_some() {
            Ok(1)
        } else {
            Ok(0)
            // Err(VMHooksError::ExecutionFailed)
        }
    }

    fn bi_get_int64(&mut self, destination_handle: RawHandle) -> Result<i64, VMHooksError> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_get_int_64)?;

        let opt_i64 = self.m_types_lock().bi_to_i64(destination_handle);

        match opt_i64 {
            Some(value) => Ok(value),
            None => {
                self.vm_error(vm_err_msg::BIG_INT_BITWISE_OPERATION_NEGATIVE)?;
                // unreachable if vm_error returns Err
                Err(VMHooksError::ExecutionFailed)
            },
        }
    }

    binary_op_method! {bi_add, add}
    binary_op_method! {bi_sub, sub}
    binary_op_method! {bi_mul, mul}
    binary_op_method! {bi_t_div, div}
    binary_op_method! {bi_t_mod, rem}

    unary_op_method! {bi_abs, abs}
    unary_op_method! {bi_neg, neg}

    fn bi_sign(&self, x: RawHandle) -> i32 {
        let bi = self.m_types_lock().bi_get(x);
        match bi.sign() {
            num_bigint::Sign::Minus => -1,
            num_bigint::Sign::NoSign => 0,
            num_bigint::Sign::Plus => 1,
        }
    }

    fn bi_cmp(&self, x: RawHandle, y: RawHandle) -> i32 {
        let bi_x = self.m_types_lock().bi_get(x);
        let bi_y = self.m_types_lock().bi_get(y);
        match bi_x.cmp(&bi_y) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    unary_op_method! {bi_sqrt, sqrt}

    fn bi_pow(&mut self, dest: RawHandle, x: RawHandle, y: RawHandle) -> Result<(), VMHooksError> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_get_int_64)?;
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_get_int_64)?;
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_set_int_64)?;

        let bi_x = self.m_types_lock().bi_get(x);
        let bi_y = self.m_types_lock().bi_get(y);
        let exp = big_int_to_i64(&bi_y).unwrap().try_into().unwrap();
        let result = pow(bi_x, exp);
        self.m_types_lock().bi_overwrite(dest, result);

        Ok(())
    }

    fn bi_log2(&self, x: RawHandle) -> i32 {
        let bi_x = self.m_types_lock().bi_get(x);
        bi_x.bits() as i32 - 1
    }

    binary_bitwise_op_method! {bi_and, bitand}
    binary_bitwise_op_method! {bi_or, bitor}
    binary_bitwise_op_method! {bi_xor, bitxor}

    fn bi_shr(&mut self, dest: RawHandle, x: RawHandle, bits: usize) -> Result<(), VMHooksError> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_get_int_64)?;
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_set_int_64)?;

        let bi_x = self.m_types_lock().bi_get(x);
        if bi_x.sign() == num_bigint::Sign::Minus {
            self.vm_error(vm_err_msg::BIG_INT_BITWISE_OPERATION_NEGATIVE)?;
        }
        let result = bi_x.shr(bits);
        self.m_types_lock().bi_overwrite(dest, result);

        Ok(())
    }

    fn bi_shl(&mut self, dest: RawHandle, x: RawHandle, bits: usize) -> Result<(), VMHooksError> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_get_int_64)?;
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_set_int_64)?;

        let bi_x = self.m_types_lock().bi_get(x);
        if bi_x.sign() == num_bigint::Sign::Minus {
            self.vm_error(vm_err_msg::BIG_INT_BITWISE_OPERATION_NEGATIVE)?;
        }
        let result = bi_x.shl(bits);
        self.m_types_lock().bi_overwrite(dest, result);

        Ok(())
    }
}
