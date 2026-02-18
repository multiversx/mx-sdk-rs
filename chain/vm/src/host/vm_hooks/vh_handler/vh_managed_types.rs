mod vh_big_float;
mod vh_big_int;
mod vh_managed_buffer;
mod vh_managed_map;

use multiversx_chain_vm_executor::VMHooksEarlyExit;
use num_traits::Signed;

use crate::{
    host::{
        context::{big_int_signed_bytes, big_int_to_i64, big_uint_to_u64, big_uint_unsigned_bytes},
        vm_hooks::{VMHooksContext, vh_early_exit::early_exit_vm_error},
    },
    types::RawHandle,
    vm_err_msg,
};

use super::VMHooksHandler;

/// Provides VM hook implementations for methods that deal with more than one type of managed type.
///
/// It is also the trait that unifies all managed type functionality.
impl<C: VMHooksContext> VMHooksHandler<C> {
    pub fn mb_to_big_int_unsigned(
        &mut self,
        buffer_handle: RawHandle,
        bi_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_to_big_int_unsigned,
        )?;

        let bytes = self.context.m_types_lock().mb_to_bytes(buffer_handle);
        self.context
            .m_types_lock()
            .bi_set_unsigned_bytes(bi_handle, bytes.as_slice());

        Ok(())
    }

    pub fn mb_to_big_int_signed(
        &mut self,
        buffer_handle: RawHandle,
        bi_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_to_big_int_unsigned,
        )?;

        let bytes = self.context.m_types_lock().mb_to_bytes(buffer_handle);
        self.context
            .m_types_lock()
            .bi_set_signed_bytes(bi_handle, bytes.as_slice());

        Ok(())
    }

    pub fn mb_from_big_int_unsigned(
        &mut self,
        buffer_handle: RawHandle,
        bi_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_from_big_int_unsigned,
        )?;

        let bi_bytes = self.context.m_types_lock().bi_get_unsigned_bytes(bi_handle);
        self.context.m_types_lock().mb_set(buffer_handle, bi_bytes);

        Ok(())
    }

    pub fn mb_from_big_int_signed(
        &mut self,
        buffer_handle: RawHandle,
        bi_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_from_big_int_signed,
        )?;

        let bi_bytes = self.context.m_types_lock().bi_get_signed_bytes(bi_handle);
        self.context.m_types_lock().mb_set(buffer_handle, bi_bytes);

        Ok(())
    }

    pub fn mb_to_small_int_unsigned(
        &self,
        buffer_handle: RawHandle,
    ) -> Result<i64, VMHooksEarlyExit> {
        let bytes = self.context.m_types_lock().mb_to_bytes(buffer_handle);
        let bu = num_bigint::BigUint::from_bytes_be(&bytes);
        if let Some(small) = big_uint_to_u64(&bu) {
            Ok(small as i64)
        } else {
            Err(early_exit_vm_error(vm_err_msg::ERROR_BYTES_EXCEED_UINT64))
        }
    }

    pub fn mb_to_small_int_signed(
        &self,
        buffer_handle: RawHandle,
    ) -> Result<i64, VMHooksEarlyExit> {
        let bytes = self.context.m_types_lock().mb_to_bytes(buffer_handle);
        let bi = num_bigint::BigInt::from_signed_bytes_be(&bytes);
        if let Some(small) = big_int_to_i64(&bi) {
            Ok(small)
        } else {
            Err(early_exit_vm_error(vm_err_msg::ERROR_BYTES_EXCEED_INT64))
        }
    }

    pub fn mb_from_small_int_unsigned(
        &self,
        buffer_handle: RawHandle,
        value: u64,
    ) -> Result<(), VMHooksEarlyExit> {
        let bu = num_bigint::BigUint::from(value);
        let bytes = big_uint_unsigned_bytes(&bu);
        self.context.m_types_lock().mb_set(buffer_handle, bytes);
        Ok(())
    }

    /// This method has a bug, it doesn't handle negative numbers correctly, it converts to the absolute value of the number.
    ///
    /// The bug will be kept here, until it is also fixed on mainnet, to allow consistent testing.
    ///
    /// The framework avoids this VM hook, starting with v0.64.2.
    pub fn mb_from_small_int_signed(
        &self,
        buffer_handle: RawHandle,
        value: i64,
    ) -> Result<(), VMHooksEarlyExit> {
        let bi = num_bigint::BigInt::from(value);
        // TODO: remove `.abs()` once the bug is fixed
        let bytes = big_int_signed_bytes(&bi.abs());
        self.context.m_types_lock().mb_set(buffer_handle, bytes);
        Ok(())
    }

    pub fn bi_to_string(
        &mut self,
        bi_handle: RawHandle,
        str_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_get_int_64)?;
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_set_bytes,
        )?;

        let bi = self.context.m_types_lock().bi_get(bi_handle);
        let s = bi.to_string();
        self.context
            .m_types_lock()
            .mb_set(str_handle, s.into_bytes());

        Ok(())
    }

    pub fn mb_set_random(
        &mut self,
        dest_handle: RawHandle,
        length: usize,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_set_random,
        )?;

        let bytes = self.context.random_next_bytes(length);
        self.mb_set(dest_handle, bytes.as_slice())?;

        Ok(())
    }
}
