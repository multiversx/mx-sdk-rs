use crate::{
    host::{
        context::big_int_to_i64,
        vm_hooks::{vh_early_exit::early_exit_vm_error, VMHooksContext, VMHooksHandler},
    },
    types::RawHandle,
    vm_err_msg,
};
use core::{
    cmp::Ordering,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Shl, Shr, Sub},
};
use multiversx_chain_vm_executor::VMHooksEarlyExit;
use num_traits::{pow, sign::Signed};
use std::convert::TryInto;

macro_rules! binary_op_method {
    ($method_name:ident, $rust_op_name:ident, $gas_cost_field:ident) => {
        pub fn $method_name(
            &mut self,
            dest: RawHandle,
            x: RawHandle,
            y: RawHandle,
        ) -> Result<(), VMHooksEarlyExit> {
            self.use_gas(self.gas_schedule().big_int_api_cost.$gas_cost_field)?;

            let bi_x = self.context.m_types_lock().bi_get(x);
            let bi_y = self.context.m_types_lock().bi_get(y);
            let result = bi_x.$rust_op_name(bi_y);
            self.context.m_types_lock().bi_overwrite(dest, result);

            Ok(())
        }
    };
}

macro_rules! binary_bitwise_op_method {
    ($method_name:ident, $rust_op_name:ident, $gas_cost_field:ident) => {
        pub fn $method_name(
            &mut self,
            dest: RawHandle,
            x: RawHandle,
            y: RawHandle,
        ) -> Result<(), VMHooksEarlyExit> {
            self.use_gas(self.gas_schedule().big_int_api_cost.$gas_cost_field)?;

            let bi_x = self.context.m_types_lock().bi_get(x);
            if bi_x.sign() == num_bigint::Sign::Minus {
                return Err(early_exit_vm_error(
                    vm_err_msg::BIG_INT_BITWISE_OPERATION_NEGATIVE,
                ));
            }
            let bi_y = self.context.m_types_lock().bi_get(y);
            if bi_y.sign() == num_bigint::Sign::Minus {
                return Err(early_exit_vm_error(
                    vm_err_msg::BIG_INT_BITWISE_OPERATION_NEGATIVE,
                ));
            }
            let result = bi_x.$rust_op_name(bi_y);
            self.context.m_types_lock().bi_overwrite(dest, result);

            Ok(())
        }
    };
}

macro_rules! unary_op_method {
    ($method_name:ident, $rust_op_name:ident, $gas_cost_field:ident) => {
        pub fn $method_name(
            &mut self,
            dest: RawHandle,
            x: RawHandle,
        ) -> Result<(), VMHooksEarlyExit> {
            self.use_gas(self.gas_schedule().big_int_api_cost.$gas_cost_field)?;

            let bi_x = self.context.m_types_lock().bi_get(x);
            let result = bi_x.$rust_op_name();
            self.context.m_types_lock().bi_overwrite(dest, result);

            Ok(())
        }
    };
}

/// Provides VM hook implementations for methods that deal big ints.
impl<C: VMHooksContext> VMHooksHandler<C> {
    pub fn bi_new(&mut self, value: i64) -> Result<RawHandle, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_new)?;

        Ok(self
            .context
            .m_types_lock()
            .bi_new_from_big_int(num_bigint::BigInt::from(value)))
    }

    pub fn bi_set_int64(
        &mut self,
        destination: RawHandle,
        value: i64,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_set_int_64)?;

        self.context
            .m_types_lock()
            .bi_overwrite(destination, num_bigint::BigInt::from(value));
        Ok(())
    }

    pub fn bi_unsigned_byte_length(
        &mut self,
        handle: RawHandle,
    ) -> Result<usize, VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .big_int_api_cost
                .big_int_unsigned_byte_length,
        )?;

        Ok(self
            .context
            .m_types_lock()
            .bi_get_unsigned_bytes(handle)
            .len())
    }

    pub fn bi_get_unsigned_bytes(
        &mut self,
        handle: RawHandle,
    ) -> Result<Vec<u8>, VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .big_int_api_cost
                .big_int_get_unsigned_bytes,
        )?;

        Ok(self.context.m_types_lock().bi_get_unsigned_bytes(handle))
    }

    pub fn bi_set_unsigned_bytes(
        &mut self,
        destination: RawHandle,
        bytes: &[u8],
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .big_int_api_cost
                .big_int_set_unsigned_bytes,
        )?;

        self.context
            .m_types_lock()
            .bi_set_unsigned_bytes(destination, bytes);

        Ok(())
    }

    pub fn bi_get_signed_bytes(&mut self, handle: RawHandle) -> Result<Vec<u8>, VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .big_int_api_cost
                .big_int_get_signed_bytes,
        )?;

        Ok(self.context.m_types_lock().bi_get_signed_bytes(handle))
    }

    pub fn bi_set_signed_bytes(
        &mut self,
        destination: RawHandle,
        bytes: &[u8],
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .big_int_api_cost
                .big_int_set_signed_bytes,
        )?;

        self.context
            .m_types_lock()
            .bi_set_signed_bytes(destination, bytes);

        Ok(())
    }

    pub fn bi_is_int64(&mut self, destination_handle: RawHandle) -> Result<i32, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_is_int_64)?;

        if self
            .context
            .m_types_lock()
            .bi_to_i64(destination_handle)
            .is_some()
        {
            Ok(1)
        } else {
            Ok(0)
        }
    }

    pub fn bi_get_int64(&mut self, destination_handle: RawHandle) -> Result<i64, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_get_int_64)?;

        let opt_i64 = self.context.m_types_lock().bi_to_i64(destination_handle);

        match opt_i64 {
            Some(value) => Ok(value),
            None => Err(early_exit_vm_error(
                vm_err_msg::BIG_INT_BITWISE_OPERATION_NEGATIVE,
            )),
        }
    }

    binary_op_method! {bi_add, add, big_int_add}
    binary_op_method! {bi_sub, sub, big_int_sub}
    binary_op_method! {bi_mul, mul, big_int_mul}
    binary_op_method! {bi_t_div, div, big_int_t_div}
    binary_op_method! {bi_t_mod, rem, big_int_t_mod}

    unary_op_method! {bi_abs, abs, big_int_abs}
    unary_op_method! {bi_neg, neg, big_int_neg}

    pub fn bi_sign(&mut self, x: RawHandle) -> Result<i32, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_sign)?;

        let bi = self.context.m_types_lock().bi_get(x);
        match bi.sign() {
            num_bigint::Sign::Minus => Ok(-1),
            num_bigint::Sign::NoSign => Ok(0),
            num_bigint::Sign::Plus => Ok(1),
        }
    }

    pub fn bi_cmp(&mut self, x: RawHandle, y: RawHandle) -> Result<i32, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_cmp)?;

        let bi_x = self.context.m_types_lock().bi_get(x);
        let bi_y = self.context.m_types_lock().bi_get(y);
        match bi_x.cmp(&bi_y) {
            Ordering::Less => Ok(-1),
            Ordering::Equal => Ok(0),
            Ordering::Greater => Ok(1),
        }
    }

    unary_op_method! {bi_sqrt, sqrt, big_int_sqrt}

    pub fn bi_pow(
        &mut self,
        dest: RawHandle,
        x: RawHandle,
        y: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_pow)?;

        let bi_x = self.context.m_types_lock().bi_get(x);
        let bi_y = self.context.m_types_lock().bi_get(y);
        let exp = big_int_to_i64(&bi_y).unwrap().try_into().unwrap();
        let result = pow(bi_x, exp);
        self.context.m_types_lock().bi_overwrite(dest, result);

        Ok(())
    }

    pub fn bi_log2(&mut self, x: RawHandle) -> Result<i32, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_log)?;

        let bi_x = self.context.m_types_lock().bi_get(x);
        Ok(bi_x.bits() as i32 - 1)
    }

    binary_bitwise_op_method! {bi_and, bitand, big_int_and}
    binary_bitwise_op_method! {bi_or, bitor, big_int_or}
    binary_bitwise_op_method! {bi_xor, bitxor, big_int_xor}

    pub fn bi_shr(
        &mut self,
        dest: RawHandle,
        x: RawHandle,
        bits: usize,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_shr)?;

        let bi_x = self.context.m_types_lock().bi_get(x);
        if bi_x.sign() == num_bigint::Sign::Minus {
            return Err(early_exit_vm_error(
                vm_err_msg::BIG_INT_BITWISE_OPERATION_NEGATIVE,
            ));
        }
        let result = bi_x.shr(bits);
        self.context.m_types_lock().bi_overwrite(dest, result);

        Ok(())
    }

    pub fn bi_shl(
        &mut self,
        dest: RawHandle,
        x: RawHandle,
        bits: usize,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_shl)?;

        let bi_x = self.context.m_types_lock().bi_get(x);
        if bi_x.sign() == num_bigint::Sign::Minus {
            return Err(early_exit_vm_error(
                vm_err_msg::BIG_INT_BITWISE_OPERATION_NEGATIVE,
            ));
        }
        let result = bi_x.shl(bits);
        self.context.m_types_lock().bi_overwrite(dest, result);

        Ok(())
    }
}
