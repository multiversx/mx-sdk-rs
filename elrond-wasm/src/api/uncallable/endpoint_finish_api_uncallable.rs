use crate::api::{EndpointFinishApi, Handle};

use super::UncallableApi;

impl EndpointFinishApi for UncallableApi {
    fn finish_slice_u8(&self, _slice: &[u8]) {
        unreachable!()
    }

    fn finish_big_int_raw(&self, _handle: Handle) {
        unreachable!()
    }

    fn finish_big_uint_raw(&self, _handle: Handle) {
        unreachable!()
    }

    fn finish_managed_buffer_raw(&self, _handle: Handle) {
        unreachable!()
    }

    fn finish_u64(&self, _value: u64) {
        unreachable!()
    }

    fn finish_i64(&self, _value: i64) {
        unreachable!()
    }
}
