mod vh_big_float;
mod vh_big_int;
mod vh_managed_buffer;
mod vh_managed_map;

use multiversx_chain_vm_executor::VMHooksError;
pub use vh_big_float::VMHooksBigFloat;
pub use vh_big_int::VMHooksBigInt;
pub use vh_managed_buffer::VMHooksManagedBuffer;
pub use vh_managed_map::VMHooksManagedMap;

use std::fmt::Debug;

use crate::types::RawHandle;

use super::VMHooksSignalError;

/// Provides VM hook implementations for methods that deal with more than one type of managed type.
///
/// It is also the trait that unifies all managed type functionality.
pub trait VMHooksManagedTypes:
    VMHooksBigInt
    + VMHooksManagedBuffer
    + VMHooksManagedMap
    + VMHooksBigFloat
    + VMHooksSignalError
    + Debug
{
    fn mb_to_big_int_unsigned(
        &mut self,
        buffer_handle: RawHandle,
        bi_handle: RawHandle,
    ) -> Result<(), VMHooksError> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_to_big_int_unsigned,
        )?;

        let bytes = self.m_types_lock().mb_to_bytes(buffer_handle);
        self.m_types_lock()
            .bi_set_unsigned_bytes(bi_handle, bytes.as_slice());

        Ok(())
    }

    fn mb_to_big_int_signed(
        &mut self,
        buffer_handle: RawHandle,
        bi_handle: RawHandle,
    ) -> Result<(), VMHooksError> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_to_big_int_unsigned,
        )?;

        let bytes = self.m_types_lock().mb_to_bytes(buffer_handle);
        self.m_types_lock()
            .bi_set_signed_bytes(bi_handle, bytes.as_slice());

        Ok(())
    }

    fn mb_from_big_int_unsigned(
        &mut self,
        buffer_handle: RawHandle,
        bi_handle: RawHandle,
    ) -> Result<(), VMHooksError> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_from_big_int_unsigned,
        )?;

        let bi_bytes = self.m_types_lock().bi_get_unsigned_bytes(bi_handle);
        self.m_types_lock().mb_set(buffer_handle, bi_bytes);

        Ok(())
    }

    fn mb_from_big_int_signed(
        &mut self,
        buffer_handle: RawHandle,
        bi_handle: RawHandle,
    ) -> Result<(), VMHooksError> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_from_big_int_signed,
        )?;

        let bi_bytes = self.m_types_lock().bi_get_signed_bytes(bi_handle);
        self.m_types_lock().mb_set(buffer_handle, bi_bytes);

        Ok(())
    }

    fn bi_to_string(
        &mut self,
        bi_handle: RawHandle,
        str_handle: RawHandle,
    ) -> Result<(), VMHooksError> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_get_int_64)?;
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_set_bytes,
        )?;

        let bi = self.m_types_lock().bi_get(bi_handle);
        let s = bi.to_string();
        self.m_types_lock().mb_set(str_handle, s.into_bytes());

        Ok(())
    }

    fn mb_set_random(&mut self, dest_handle: RawHandle, length: usize) -> Result<(), VMHooksError> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_set_random,
        )?;

        let bytes = self.random_next_bytes(length);
        self.mb_set(dest_handle, bytes.as_slice());

        Ok(())
    }
}
