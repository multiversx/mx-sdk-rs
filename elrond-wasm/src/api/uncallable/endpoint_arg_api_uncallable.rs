use crate::api::{EndpointArgumentApi, ErrorApi, Handle};
use crate::err_msg;
use crate::types::BoxedBytes;
use alloc::vec::Vec;

use super::UncallableApi;

impl EndpointArgumentApi for UncallableApi {
    fn get_num_arguments(&self) -> i32 {
        unreachable!()
    }

    fn get_argument_len(&self, arg_index: i32) -> usize {
        unreachable!()
    }

    fn copy_argument_to_slice(&self, arg_index: i32, slice: &mut [u8]) {
        unreachable!()
    }

    fn get_argument_vec_u8(&self, arg_index: i32) -> Vec<u8> {
        unreachable!()
    }

    fn get_argument_big_int_raw(&self, arg_id: i32) -> Handle {
        unreachable!()
    }

    fn get_argument_big_uint_raw(&self, arg_id: i32) -> Handle {
        unreachable!()
    }

    fn get_argument_managed_buffer_raw(&self, arg_id: i32) -> Handle {
        unreachable!()
    }

    fn get_argument_u64(&self, arg_id: i32) -> u64 {
        unreachable!()
    }

    fn get_argument_i64(&self, arg_id: i32) -> i64 {
        unreachable!()
    }
}
