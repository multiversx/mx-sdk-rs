multiversx_sc::imports!();

/// Serialization of managed types, mixed with the heap allocator.
#[multiversx_sc::module]
pub trait EchoManagedTypesWithAlloc {
    /// This tests that nested serialization of managed buffers within unmanaged types works.
    #[endpoint]
    fn echo_vec_of_managed_buffer(&self, mb: Vec<ManagedBuffer>) -> Vec<ManagedBuffer> {
        mb
    }

    /// This tests that nested serialization of big ints within unmanaged types works.
    #[endpoint]
    fn echo_big_int_vec(&self, x: Vec<BigInt>) -> Vec<BigInt> {
        x
    }

    #[endpoint]
    fn echo_varags_u32(&self, m: MultiValueVec<u32>) -> MultiValue2<usize, MultiValueVec<u32>> {
        let v = m.into_vec();
        (v.len(), v.into()).into()
    }

    #[endpoint]
    fn echo_varags_big_uint(&self, m: MultiValueVec<BigUint>) -> MultiValueVec<BigUint> {
        m.into_vec().into()
    }
}
