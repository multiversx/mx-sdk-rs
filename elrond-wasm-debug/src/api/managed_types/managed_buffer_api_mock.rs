use elrond_wasm::{
    api::{Handle, ManagedBufferApi},
    types::BoxedBytes,
};

use crate::TxContext;

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
        _source_handle: Handle,
        _starting_position: usize,
        _dest_slice: &mut [u8],
    ) -> bool {
        unreachable!()
    }

    fn mb_copy_slice(
        &self,
        _source_handle: Handle,
        _starting_pos: usize,
        _slice_len: usize,
        _dest_handle: Handle,
    ) -> bool {
        unreachable!()
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
