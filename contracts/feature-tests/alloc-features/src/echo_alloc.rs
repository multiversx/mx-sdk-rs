multiversx_sc::imports!();

use crate::types::*;

// String is not part of the standard imports because we want to discourage its use
use multiversx_sc::types::String;

/// Test serialization for heap-allocated types.
#[multiversx_sc::module]
pub trait EchoAllocTypes {
    #[endpoint]
    fn echo_h256(&self, h: H256) -> H256 {
        h
    }

    #[endpoint]
    fn echo_boxed_array_u8(&self, s: Box<[u8; 128]>) -> Box<[u8; 128]> {
        s
    }

    #[endpoint]
    fn echo_boxed_bytes(&self, arg: BoxedBytes) -> MultiValue2<BoxedBytes, usize> {
        let l = arg.len();
        (arg, l).into()
    }

    #[endpoint]
    fn echo_slice_u8<'s>(&self, slice: &'s [u8]) -> MultiValue2<&'s [u8], usize> {
        let l = slice.len();
        (slice, l).into()
    }

    #[endpoint]
    fn echo_vec_u8(&self, arg: Vec<u8>) -> MultiValue2<Vec<u8>, usize> {
        let l = arg.len();
        (arg, l).into()
    }

    #[endpoint]
    fn echo_string(&self, s: String) -> MultiValue2<String, usize> {
        let l = s.len();
        (s, l).into()
    }

    #[endpoint]
    fn echo_str<'s>(&self, s: &'s str) -> MultiValue2<&'s str, usize> {
        let l = s.len();
        (s, l).into()
    }

    #[endpoint]
    fn echo_str_box(&self, s: Box<str>) -> MultiValue2<Box<str>, usize> {
        let l = s.len();
        (s, l).into()
    }

    #[endpoint]
    fn echo_async_result_empty(&self, a: AsyncCallResult<()>) {
        if let AsyncCallResult::Err(msg) = a {
            sc_panic!(msg.err_msg);
        }
    }

    #[endpoint]
    fn echo_large_boxed_byte_array(&self, lbba: LargeBoxedByteArray) -> LargeBoxedByteArray {
        lbba
    }

    #[endpoint]
    fn echo_boxed_ser_example_1(&self, se: Box<StructExampleAlloc>) -> Box<StructExampleAlloc> {
        se
    }

    #[endpoint]
    fn echo_multi_value_tuples(
        &self,
        m: MultiValueVec<MultiValue2<isize, Vec<u8>>>,
    ) -> MultiValueVec<MultiValue2<isize, Vec<u8>>> {
        let mut result: Vec<MultiValue2<isize, Vec<u8>>> = Vec::new();
        for m_arg in m.into_vec().into_iter() {
            result.push(m_arg.into_tuple().into())
        }
        result.into()
    }

    #[endpoint]
    fn echo_ser_example_1(&self, se: StructExampleAlloc) -> StructExampleAlloc {
        se
    }
}
