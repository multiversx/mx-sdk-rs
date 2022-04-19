use crate::{
    api::{endpoint_arg_api::EndpointArgumentApiImpl, EndpointArgumentApi, Handle},
    types::heap::BoxedBytes,
};

use super::UncallableApi;

impl EndpointArgumentApi for UncallableApi {
    type EndpointArgumentApiImpl = UncallableApi;

    fn argument_api_impl() -> Self::EndpointArgumentApiImpl {
        unreachable!()
    }
}

impl EndpointArgumentApiImpl for UncallableApi {
    fn get_num_arguments(&self) -> i32 {
        unreachable!()
    }

    fn get_argument_len(&self, _arg_index: i32) -> usize {
        unreachable!()
    }

    fn copy_argument_to_slice(&self, _arg_index: i32, _slice: &mut [u8]) {
        unreachable!()
    }

    fn get_argument_boxed_bytes(&self, _arg_index: i32) -> BoxedBytes {
        unreachable!()
    }

    fn load_argument_big_int_signed(&self, _arg_id: i32, _dest: Handle) {
        unreachable!()
    }

    fn load_argument_big_int_unsigned(&self, _arg_id: i32, _dest: Handle) {
        unreachable!()
    }

    fn load_argument_managed_buffer(&self, _arg_id: i32, _dest: Handle) {
        unreachable!()
    }

    fn get_argument_u64(&self, _arg_id: i32) -> u64 {
        unreachable!()
    }

    fn get_argument_i64(&self, _arg_id: i32) -> i64 {
        unreachable!()
    }
}
