multiversx_sc::imports!();

/// Various features of heap-allocated types.
#[multiversx_sc::module]
pub trait AllocTypeFeatures {
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
    fn boxed_bytes_split(&self, bb: BoxedBytes, at: usize) -> MultiValue2<BoxedBytes, BoxedBytes> {
        bb.split(at).into()
    }

    // VEC OPERATIONS

    #[view]
    fn vec_concat_const(&self) -> Vec<u8> {
        let mut result = b"part1".to_vec();
        result.extend_from_slice(&[0u8; 100][..]);
        result
    }
}
