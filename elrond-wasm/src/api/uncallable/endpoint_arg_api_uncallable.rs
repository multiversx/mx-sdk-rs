use crate::api::{EndpointArgumentApi, Handle};
use alloc::vec::Vec;

use super::UncallableApi;

impl EndpointArgumentApi for UncallableApi {
    fn get_num_arguments(&self) -> i32 {
        unreachable!()
    }

    fn get_argument_len(&self, _arg_index: i32) -> usize {
        unreachable!()
    }

    fn copy_argument_to_slice(&self, _arg_index: i32, _slice: &mut [u8]) {
        unreachable!()
    }

    fn get_argument_vec_u8(&self, _arg_index: i32) -> Vec<u8> {
        unreachable!()
    }

    fn get_argument_big_int_raw(&self, _arg_id: i32) -> Handle {
        unreachable!()
    }

    fn get_argument_big_uint_raw(&self, _arg_id: i32) -> Handle {
        unreachable!()
    }

    fn get_argument_managed_buffer_raw(&self, _arg_id: i32) -> Handle {
        unreachable!()
    }

    fn get_argument_u64(&self, _arg_id: i32) -> u64 {
        unreachable!()
    }

    fn get_argument_i64(&self, _arg_id: i32) -> i64 {
        unreachable!()
    }
}
