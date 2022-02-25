elrond_wasm::imports!();

use crate::types::*;
use core::num::NonZeroUsize;

// String is not part of the standard imports because we want to discourage its use
use elrond_wasm::String;

/// Test endpoint argument and result serialization.
#[elrond_wasm::module]
pub trait EchoTypes {
    #[endpoint]
    fn echo_u64(&self, i: u64) -> u64 {
        i
    }

    #[endpoint]
    fn echo_i64(&self, i: i64) -> i64 {
        i
    }

    #[endpoint]
    fn echo_i32(&self, i: i32) -> i32 {
        i
    }

    #[endpoint]
    fn echo_u32(&self, i: u32) -> u32 {
        i
    }

    #[endpoint]
    fn echo_isize(&self, i: isize) -> isize {
        i
    }

    #[endpoint]
    fn echo_usize(&self, i: usize) -> usize {
        i
    }

    #[endpoint]
    fn echo_i8(&self, i: i8) -> i8 {
        i
    }

    #[endpoint]
    fn echo_u8(&self, i: u8) -> u8 {
        i
    }

    #[endpoint]
    fn echo_bool(&self, i: bool) -> bool {
        i
    }

    #[endpoint]
    fn echo_opt_bool(&self, i: Option<bool>) -> Option<bool> {
        i
    }

    #[endpoint]
    fn echo_h256(&self, h: H256) -> H256 {
        h
    }

    #[endpoint]
    fn echo_nothing(&self, #[var_args] nothing: ()) -> () {
        nothing
    }

    #[endpoint]
    fn echo_array_u8(&self, s: [u8; 5]) -> [u8; 5] {
        s
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
    fn echo_varags_u32(
        &self,
        #[var_args] m: MultiValueVec<u32>,
    ) -> MultiValue2<usize, MultiValueVec<u32>> {
        let v = m.into_vec();
        (v.len(), v.into()).into()
    }

    #[endpoint]
    fn take_varags_u32(&self, #[var_args] m: MultiValueVec<u32>) -> usize {
        let v = m.into_vec();
        v.len()
    }

    #[endpoint]
    fn echo_varags_big_uint(
        &self,
        #[var_args] m: MultiValueVec<BigUint>,
    ) -> MultiValueVec<BigUint> {
        m.into_vec().into()
    }

    #[endpoint]
    fn echo_varags_tuples(
        &self,
        #[var_args] m: MultiValueVec<MultiValue2<isize, Vec<u8>>>,
    ) -> MultiValueVec<MultiValue2<isize, Vec<u8>>> {
        let mut result: Vec<MultiValue2<isize, Vec<u8>>> = Vec::new();
        for m_arg in m.into_vec().into_iter() {
            result.push(m_arg.into_tuple().into())
        }
        result.into()
    }

    #[endpoint]
    fn echo_async_result_empty(&self, #[var_args] a: AsyncCallResult<()>) {
        if let AsyncCallResult::Err(msg) = a {
            sc_panic!(msg.err_msg.as_slice());
        }
    }

    #[endpoint]
    fn echo_large_boxed_byte_array(&self, lbba: LargeBoxedByteArray) -> LargeBoxedByteArray {
        lbba
    }

    #[endpoint]
    fn echo_ser_example_1(&self, se: SerExample1) -> SerExample1 {
        se
    }

    #[endpoint]
    fn echo_boxed_ser_example_1(&self, se: Box<SerExample1>) -> Box<SerExample1> {
        se
    }

    #[endpoint]
    fn echo_ser_example_2(&self, se: SerExample2) -> SerExample2 {
        se
    }

    #[endpoint]
    fn echo_boxed_ser_example_2(&self, se: Box<SerExample2>) -> Box<SerExample2> {
        se
    }

    #[view]
    fn echo_simple_enum(&self, se: SimpleEnum) -> SimpleEnum {
        se
    }

    #[view]
    fn finish_simple_enum_variant_1(&self) -> SimpleEnum {
        SimpleEnum::Variant1
    }

    #[view]
    fn echo_non_zero_usize(&self, nz: NonZeroUsize) -> NonZeroUsize {
        nz
    }

    #[view]
    fn echo_some_args_ignore_others(
        &self,
        i: i32,
        #[var_args] opt: OptionalValue<i32>,
        #[var_args] _ignore: IgnoreValue,
    ) -> MultiValue2<i32, OptionalValue<i32>> {
        (i, opt).into()
    }

    #[view]
    fn echo_arrayvec(&self, av: ArrayVec<i32, 3>) -> ArrayVec<i32, 3> {
        av
    }
}
