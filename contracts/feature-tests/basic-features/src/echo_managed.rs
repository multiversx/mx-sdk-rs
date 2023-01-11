multiversx_sc::imports!();

/// Test endpoint argument and result serialization.
#[multiversx_sc::module]
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

    /// This tests that nested serialization of big ints within unmanaged types works.
    #[endpoint]
    fn echo_big_int_managed_vec(&self, x: ManagedVec<BigInt>) -> ManagedVec<BigInt> {
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
    ) -> MultiValue2<ManagedAddress, ManagedVec<Self::Api, ManagedBuffer>> {
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

    #[endpoint]
    fn echo_managed_async_result_empty(&self, a: ManagedAsyncCallResult<()>) {
        if let ManagedAsyncCallResult::Err(msg) = a {
            sc_panic!(msg.err_msg)
        }
    }

    #[endpoint]
    fn echo_varags_managed_eager(
        &self,
        m: MultiValueManagedVec<Self::Api, u32>,
    ) -> MultiValue2<usize, MultiValueManagedVec<Self::Api, u32>> {
        let v = m.into_vec();
        (v.len(), v.into()).into()
    }

    #[endpoint]
    fn echo_varags_managed_sum(
        &self,
        m: MultiValueEncoded<MultiValue2<u32, u32>>,
    ) -> MultiValueEncoded<MultiValue3<u32, u32, u32>> {
        let mut result = MultiValueEncoded::new();
        for arg in m.into_iter() {
            let (x, y) = arg.into_tuple();
            result.push((x, y, x + y).into())
        }
        result
    }
}
