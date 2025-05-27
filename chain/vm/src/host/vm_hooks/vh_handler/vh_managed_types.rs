mod vh_big_float;
mod vh_big_int;
mod vh_managed_buffer;
mod vh_managed_map;

use multiversx_chain_vm_executor::VMHooksEarlyExit;

use crate::{host::vm_hooks::VMHooksContext, types::RawHandle};

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
