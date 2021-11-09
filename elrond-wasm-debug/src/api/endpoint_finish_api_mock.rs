use crate::DebugApi;
use elrond_wasm::api::{BigIntApi, EndpointFinishApi, Handle, ManagedBufferApi};
use num_bigint::{BigInt, BigUint};

/// Interface to only be used by code generated by the macros.
/// The smart contract code doesn't have access to these methods directly.
impl EndpointFinishApi for DebugApi {
    fn finish_slice_u8(&self, slice: &[u8]) {
        let mut v = vec![0u8; slice.len()];
        v.copy_from_slice(slice);
        let mut tx_result = self.result_borrow_mut();
        tx_result.result_values.push(v)
    }

    fn finish_big_int_raw(&self, handle: Handle) {
        let bi_bytes = self.bi_get_signed_bytes(handle);
        let mut tx_result = self.result_borrow_mut();
        tx_result.result_values.push(bi_bytes.into_vec());
    }

    fn finish_big_uint_raw(&self, handle: Handle) {
        let bu_bytes = self.bi_get_unsigned_bytes(handle);
        let mut tx_result = self.result_borrow_mut();
        tx_result.result_values.push(bu_bytes.into_vec());
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

    fn finish_big_float(&self, handle: Handle) {
        let managed_types = self.m_types_borrow();
        let bf = managed_types.big_float_map.get(handle);
        let bf_bytes = &bf.to_be_bytes()[..];
        self.finish_slice_u8(bf_bytes)
    }
}
