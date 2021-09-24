elrond_wasm::imports!();

/// Test endpoint argument and result serialization.
#[elrond_wasm::module]
pub trait EchoManagedTypes {
    #[endpoint]
    fn echo_big_uint(&self, bi: BigUint) -> BigUint {
        bi
    }

    #[endpoint]
    fn echo_big_int(&self, bi: BigInt) -> BigInt {
        bi
    }

    #[endpoint]
    fn echo_managed_buffer(&self, mb: ManagedBuffer) -> ManagedBuffer {
        mb
    }

    #[endpoint]
    fn echo_managed_address(&self, ma: ManagedAddress) -> ManagedAddress {
        ma
    }

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

    /// This tests that nested serialization of big ints within unmanaged types works.
    #[endpoint]
    fn echo_big_int_tuple(&self, x: (BigInt, ManagedBuffer)) -> (BigInt, ManagedBuffer) {
        x
    }

    /// This tests that nested serialization of big ints within unmanaged types works.
    #[endpoint]
    fn echo_big_int_option(&self, x: Option<BigInt>) -> Option<BigInt> {
        x
    }

    #[endpoint]
    fn echo_tuple_into_multiresult(
        &self,
        addr: ManagedAddress,
        vec: ManagedVec<Self::Api, ManagedBuffer>,
    ) -> MultiResult2<ManagedAddress, ManagedVec<Self::Api, ManagedBuffer>> {
        (addr, vec).into()
    }

    #[endpoint]
    fn echo_managed_vec_of_managed_vec(
        &self,
        mv: ManagedVec<ManagedVec<usize>>,
    ) -> ManagedVec<ManagedVec<usize>> {
        mv
    }

    #[endpoint]
    fn echo_managed_vec_of_token_identifier(
        &self,
        mv: ManagedVec<TokenIdentifier>,
    ) -> ManagedVec<TokenIdentifier> {
        mv
    }
}
