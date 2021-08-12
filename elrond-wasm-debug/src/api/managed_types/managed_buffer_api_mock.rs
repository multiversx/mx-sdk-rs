use elrond_wasm::{
    api::{Handle, ManagedBufferApi},
    types::BoxedBytes,
};

use crate::TxContext;

impl ManagedBufferApi for TxContext {
    fn new_empty(&self) -> Handle {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        tx_output
            .managed_types
            .managed_buffer_map
            .insert_new_handle(Vec::new())
    }

    fn new_from_bytes(&self, bytes: &[u8]) -> Handle {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        tx_output
            .managed_types
            .managed_buffer_map
            .insert_new_handle(Vec::from(bytes))
    }

    fn len(&self, handle: Handle) -> usize {
        let tx_output = self.tx_output_cell.borrow();
        let data = tx_output.managed_types.managed_buffer_map.get(handle);
        data.len()
    }

    fn overwrite(&self, handle: Handle, value: &[u8]) {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        tx_output
            .managed_types
            .managed_buffer_map
            .insert(handle, value.into());
    }

    fn extend_from_slice(&self, handle: Handle, slice: &[u8]) {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        let data = tx_output.managed_types.managed_buffer_map.get_mut(handle);
        data.extend_from_slice(slice);
    }

    fn to_boxed_bytes(&self, handle: Handle) -> BoxedBytes {
        let tx_output = self.tx_output_cell.borrow();
        let data = tx_output.managed_types.managed_buffer_map.get(handle);
        data.into()
    }
}
