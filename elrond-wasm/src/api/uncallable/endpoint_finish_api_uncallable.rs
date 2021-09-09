use crate::api::{EndpointFinishApi, Handle};

use super::UncallableApi;

impl EndpointFinishApi for UncallableApi {
    fn finish_slice_u8(&self, slice: &[u8]) {
        unreachable!()
    }

    fn finish_big_int_raw(&self, handle: Handle) {
        unreachable!()
    }

    fn finish_big_uint_raw(&self, handle: Handle) {
        unreachable!()
    }

    fn finish_managed_buffer_raw(&self, handle: Handle) {
        unreachable!()
    }

    fn finish_u64(&self, value: u64) {
        unreachable!()
    }

    fn finish_i64(&self, value: i64) {
        unreachable!()
    }
}
