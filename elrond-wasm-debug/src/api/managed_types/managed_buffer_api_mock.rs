use elrond_wasm::{
    api::{Handle, InvalidSliceError, ManagedBufferApi},
    types::BoxedBytes,
};

use crate::TxContext;

impl TxContext {
    fn mb_get_slice(
        &self,
        source_handle: Handle,
        starting_position: usize,
        slice_len: usize,
    ) -> Option<Vec<u8>> {
        let tx_output = self.tx_output_cell.borrow();
        let all_bytes = tx_output
            .managed_types
            .managed_buffer_map
            .get(source_handle);
        if starting_position + slice_len <= all_bytes.len() {
            Some(all_bytes[starting_position..starting_position + slice_len].to_vec())
        } else {
            None
        }
    }
}

impl ManagedBufferApi for TxContext {
    fn mb_new_empty(&self) -> Handle {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        tx_output
            .managed_types
            .managed_buffer_map
            .insert_new_handle(Vec::new())
    }

    fn mb_new_from_bytes(&self, bytes: &[u8]) -> Handle {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        tx_output
            .managed_types
            .managed_buffer_map
            .insert_new_handle(Vec::from(bytes))
    }

    fn mb_len(&self, handle: Handle) -> usize {
        let tx_output = self.tx_output_cell.borrow();
        let data = tx_output.managed_types.managed_buffer_map.get(handle);
        data.len()
    }

    fn mb_to_boxed_bytes(&self, handle: Handle) -> BoxedBytes {
        let tx_output = self.tx_output_cell.borrow();
        let data = tx_output.managed_types.managed_buffer_map.get(handle);
        data.into()
    }

    fn mb_load_slice(
        &self,
        source_handle: Handle,
        starting_position: usize,
        dest_slice: &mut [u8],
    ) -> Result<(), InvalidSliceError> {
        let opt_slice = self.mb_get_slice(source_handle, starting_position, dest_slice.len());
        if let Some(slice) = opt_slice {
            dest_slice.copy_from_slice(slice.as_slice());
            Ok(())
        } else {
            Err(InvalidSliceError)
        }
    }

    fn mb_copy_slice(
        &self,
        source_handle: Handle,
        starting_position: usize,
        slice_len: usize,
        dest_handle: Handle,
    ) -> Result<(), InvalidSliceError> {
        let opt_slice = self.mb_get_slice(source_handle, starting_position, slice_len);
        if let Some(slice) = opt_slice {
            let mut tx_output = self.tx_output_cell.borrow_mut();
            tx_output
                .managed_types
                .managed_buffer_map
                .insert(dest_handle, slice);
            Ok(())
        } else {
            Err(InvalidSliceError)
        }
    }

    fn mb_copy_to_slice_pad_right(&self, handle: Handle, destination: &mut [u8]) {
        let bytes = self.mb_to_boxed_bytes(handle);
        let offset = 32 - bytes.len();
        destination[offset..].clone_from_slice(bytes.as_slice());
    }

    fn mb_overwrite(&self, handle: Handle, value: &[u8]) {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        tx_output
            .managed_types
            .managed_buffer_map
            .insert(handle, value.into());
    }

    fn mb_append(&self, accumulator_handle: Handle, data_handle: Handle) {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        let mut data = tx_output
            .managed_types
            .managed_buffer_map
            .get(data_handle)
            .clone();
        let accumulator = tx_output
            .managed_types
            .managed_buffer_map
            .get_mut(accumulator_handle);
        accumulator.append(&mut data);
    }

    fn mb_append_bytes(&self, accumulator_handle: Handle, bytes: &[u8]) {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        let accumulator = tx_output
            .managed_types
            .managed_buffer_map
            .get_mut(accumulator_handle);
        accumulator.extend_from_slice(bytes);
    }
}
