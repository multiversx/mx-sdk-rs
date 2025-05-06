use multiversx_chain_vm_executor::VMHooksError;

use crate::host::context::InvalidSliceError;
use crate::types::RawHandle;

use crate::host::vm_hooks::VMHooksHandlerSource;

/// Provides VM hook implementations for methods that deal managed buffers.
pub trait VMHooksManagedBuffer: VMHooksHandlerSource {
    fn mb_new_empty(&self) -> RawHandle {
        self.m_types_lock().mb_new(Vec::new())
    }

    fn mb_new_from_bytes(&self, bytes: &[u8]) -> RawHandle {
        self.m_types_lock().mb_new(Vec::from(bytes))
    }

    fn mb_len(&self, handle: RawHandle) -> usize {
        self.m_types_lock().mb_len(handle)
    }

    fn mb_set(&self, handle: RawHandle, value: &[u8]) {
        self.m_types_lock().mb_set(handle, value.to_vec());
    }

    fn mb_get_bytes(&self, source_handle: RawHandle) -> Vec<u8> {
        self.m_types_lock().mb_get(source_handle).to_vec()
    }

    fn mb_get_slice(
        &self,
        source_handle: RawHandle,
        starting_position: usize,
        slice_len: usize,
    ) -> Result<Vec<u8>, InvalidSliceError> {
        self.m_types_lock()
            .mb_get_slice(source_handle, starting_position, slice_len)
    }

    fn mb_copy_slice(
        &self,
        source_handle: RawHandle,
        starting_position: usize,
        slice_len: usize,
        dest_handle: RawHandle,
    ) -> i32 {
        let result = self
            .m_types_lock()
            .mb_get_slice(source_handle, starting_position, slice_len);
        if let Ok(slice) = result {
            self.m_types_lock().mb_set(dest_handle, slice);
            0
        } else {
            1
        }
    }

    fn mb_set_slice(
        &self,
        dest_handle: RawHandle,
        starting_position: usize,
        source_slice: &[u8],
    ) -> i32 {
        let result = self
            .m_types_lock()
            .mb_set_slice(dest_handle, starting_position, source_slice);
        if result.is_ok() {
            0
        } else {
            1
        }
    }

    fn mb_append(&self, accumulator_handle: RawHandle, data_handle: RawHandle) {
        let mut data = self.m_types_lock().mb_get(data_handle).to_vec();
        self.m_types_lock()
            .mb_update(accumulator_handle, |accumulator| {
                accumulator.append(&mut data);
            });
    }

    fn mb_append_bytes(&self, accumulator_handle: RawHandle, bytes: &[u8]) {
        self.m_types_lock()
            .mb_append_bytes(accumulator_handle, bytes);
    }

    fn mb_eq(&self, handle1: RawHandle, handle2: RawHandle) -> i32 {
        let managed_types = self.m_types_lock();
        let bytes1 = managed_types.mb_get(handle1);
        let bytes2 = managed_types.mb_get(handle2);
        if bytes1 == bytes2 {
            1
        } else {
            0
        }
    }

    fn mb_to_hex(
        &mut self,
        source_handle: RawHandle,
        dest_handle: RawHandle,
    ) -> Result<(), VMHooksError> {
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

        let encoded = hex::encode(self.m_types_lock().mb_get(source_handle));
        self.m_types_lock()
            .mb_set(dest_handle, encoded.into_bytes());
        Ok(())
    }
}
