elrond_wasm::imports!();

use core::num::NonZeroUsize;

/// Exposes various methods of various types provided by elrond-wasm.
#[elrond_wasm::module]
pub trait TypeFeatures {
    #[view]
    fn non_zero_usize_iter(&self, how_many: usize) -> MultiValueEncoded<NonZeroUsize> {
        let mut result = MultiValueEncoded::new();
        for nz in NonZeroUsizeIterator::from_1_to_n(how_many) {
            result.push(nz);
        }
        result
    }

    #[view]
    fn non_zero_usize_macro(&self, number: usize) -> NonZeroUsize {
        non_zero_usize!(number, "wants non-zero")
    }
}
