use multiversx_chain_vm_executor::{MemPtr, VMHooksEarlyExit};

use crate::host::vm_hooks::vh_dispatcher::{RESULT_ERROR, RESULT_OK};
use crate::host::vm_hooks::{VMHooksHandler, VMHooksHandlerSource};
use crate::types::RawHandle;

/// Provides VM hook implementations for methods that deal managed buffers.
impl<C: VMHooksHandlerSource> VMHooksHandler<C> {
    pub fn mb_new_empty(&mut self) -> Result<RawHandle, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().managed_buffer_api_cost.m_buffer_new)?;

        Ok(self.context.m_types_lock().mb_new(Vec::new()))
    }

    pub fn mb_new_from_bytes(&mut self, bytes: &[u8]) -> Result<RawHandle, VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_new_from_bytes,
        )?;

        Ok(self.context.m_types_lock().mb_new(Vec::from(bytes)))
    }

    pub fn mb_len(&mut self, handle: RawHandle) -> Result<usize, VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_get_length,
        )?;

        Ok(self.context.m_types_lock().mb_len(handle))
    }

    pub fn mb_set(&mut self, handle: RawHandle, value: &[u8]) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_set_bytes,
        )?;

        self.context.m_types_lock().mb_set(handle, value.to_vec());

        Ok(())
    }

    pub fn mb_get_bytes(&mut self, source_handle: RawHandle) -> Result<Vec<u8>, VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_get_bytes,
        )?;

        Ok(self.context.m_types_lock().mb_get(source_handle).to_vec())
    }

    pub fn mb_get_slice(
        &mut self,
        source_handle: RawHandle,
        starting_position: usize,
        slice_length: usize,
        result_offset: MemPtr,
    ) -> Result<i32, VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_get_byte_slice,
        )?;

        if let Ok(bytes) =
            self.context
                .m_types_lock()
                .mb_get_slice(source_handle, starting_position, slice_length)
        {
            assert_eq!(bytes.len(), slice_length);
            unsafe {
                self.context.memory_store(result_offset, &bytes);
            }
            Ok(RESULT_OK)
        } else {
            Ok(RESULT_ERROR)
        }
    }

    pub fn mb_copy_slice(
        &mut self,
        source_handle: RawHandle,
        starting_position: usize,
        slice_len: usize,
        dest_handle: RawHandle,
    ) -> Result<i32, VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_copy_byte_slice,
        )?;

        let result =
            self.context
                .m_types_lock()
                .mb_get_slice(source_handle, starting_position, slice_len);
        if let Ok(slice) = result {
            self.context.m_types_lock().mb_set(dest_handle, slice);
            Ok(RESULT_OK)
        } else {
            Ok(RESULT_ERROR)
        }
    }

    pub fn mb_set_slice(
        &mut self,
        dest_handle: RawHandle,
        starting_position: usize,
        source_slice: &[u8],
    ) -> Result<i32, VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_set_bytes,
        )?;

        let result =
            self.context
                .m_types_lock()
                .mb_set_slice(dest_handle, starting_position, source_slice);
        if result.is_ok() {
            Ok(RESULT_OK)
        } else {
            Ok(RESULT_ERROR)
        }
    }

    pub fn mb_append(
        &mut self,
        accumulator_handle: RawHandle,
        data_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().managed_buffer_api_cost.m_buffer_append)?;

        let mut data = self.context.m_types_lock().mb_get(data_handle).to_vec();
        self.context
            .m_types_lock()
            .mb_update(accumulator_handle, |accumulator| {
                accumulator.append(&mut data);
            });

        Ok(())
    }

    pub fn mb_append_bytes(
        &mut self,
        accumulator_handle: RawHandle,
        bytes: &[u8],
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_append_bytes,
        )?;

        self.context
            .m_types_lock()
            .mb_append_bytes(accumulator_handle, bytes);

        Ok(())
    }

    pub fn mb_eq(
        &mut self,
        handle1: RawHandle,
        handle2: RawHandle,
    ) -> Result<i32, VMHooksEarlyExit> {
        self.use_gas(
            2 * self
                .gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_get_bytes,
        )?;

        let managed_types = self.context.m_types_lock();
        let bytes1 = managed_types.mb_get(handle1);
        let bytes2 = managed_types.mb_get(handle2);
        if bytes1 == bytes2 {
            Ok(RESULT_ERROR)
        } else {
            Ok(RESULT_OK)
        }
    }

    pub fn mb_to_hex(
        &mut self,
        source_handle: RawHandle,
        dest_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_get_bytes,
        )?;

        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_set_bytes,
        )?;

        let encoded = hex::encode(self.context.m_types_lock().mb_get(source_handle));
        self.context
            .m_types_lock()
            .mb_set(dest_handle, encoded.into_bytes());
        Ok(())
    }
}
