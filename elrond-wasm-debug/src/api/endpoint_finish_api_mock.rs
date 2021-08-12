use crate::TxContext;
use elrond_wasm::api::{EndpointFinishApi, Handle, ManagedBufferApi};
use num_bigint::{BigInt, BigUint};

/// Interface to only be used by code generated by the macros.
/// The smart contract code doesn't have access to these methods directly.
impl EndpointFinishApi for TxContext {
    fn finish_slice_u8(&self, slice: &[u8]) {
        let mut v = vec![0u8; slice.len()];
        v.copy_from_slice(slice);
        let mut tx_output = self.tx_output_cell.borrow_mut();
        tx_output.result.result_values.push(v)
    }

    fn finish_big_int_raw(&self, handle: Handle) {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        let bi = tx_output.managed_types.big_int_map.get(handle);
        let bytes = bi.to_signed_bytes_be();
        tx_output.result.result_values.push(bytes);
    }

    fn finish_big_uint_raw(&self, handle: Handle) {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        let bi = tx_output.managed_types.big_int_map.get(handle);
        let (_, bytes) = bi.to_bytes_be();
        tx_output.result.result_values.push(bytes);
    }

    fn finish_managed_buffer_raw(&self, handle: Handle) {
        let bytes = self.mb_to_boxed_bytes(handle);
        self.finish_slice_u8(bytes.as_slice());
    }

    fn finish_i64(&self, value: i64) {
        if value == 0 {
            self.finish_slice_u8(&[]);
        } else {
            self.finish_slice_u8(BigInt::from(value).to_signed_bytes_be().as_slice());
        }
    }

    fn finish_u64(&self, value: u64) {
        if value == 0 {
            self.finish_slice_u8(&[]);
        } else {
            self.finish_slice_u8(BigUint::from(value).to_bytes_be().as_slice());
        }
    }
}
