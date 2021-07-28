elrond_wasm::imports!();

use core::num::NonZeroUsize;

/// Exposes various methods of various types provided by elrond-wasm.
#[elrond_wasm::module]
pub trait TypeFeatures {
    // H256

    #[endpoint]
    fn compare_h256(&self, h1: H256, h2: H256) -> bool {
        h1 == h2
    }

    #[endpoint]
    fn h256_is_zero(&self, h: H256) -> bool {
        h.is_zero()
    }

    // BOXED BYTES

    #[endpoint]
    fn boxed_bytes_zeros(&self, len: usize) -> BoxedBytes {
        BoxedBytes::zeros(len)
    }

    #[endpoint]
    fn boxed_bytes_concat_2(&self, slice1: &[u8], slice2: &[u8]) -> BoxedBytes {
        BoxedBytes::from_concat(&[slice1, slice2][..])
    }

    #[endpoint]
    fn boxed_bytes_split(&self, bb: BoxedBytes, at: usize) -> MultiResult2<BoxedBytes, BoxedBytes> {
        bb.split(at).into()
    }

    // VEC OPERATIONS

    #[view]
    fn vec_concat_const(&self) -> Vec<u8> {
        let mut result = b"part1".to_vec();
        result.extend_from_slice(&[0u8; 100][..]);
        result
    }

    // NON ZERO EXTRA

    #[view]
    fn non_zero_usize_iter(&self, how_many: usize) -> MultiResultVec<NonZeroUsize> {
        let mut result = Vec::<NonZeroUsize>::new();
        for nz in NonZeroUsizeIterator::from_1_to_n(how_many) {
            result.push(nz);
        }
        result.into()
    }

    #[view]
    fn non_zero_usize_macro(&self, number: usize) -> SCResult<NonZeroUsize> {
        let nz = non_zero_usize!(number, "wans non-zero");
        Ok(nz)
    }
}
