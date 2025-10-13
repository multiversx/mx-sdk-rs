use multiversx_sc::imports::*;

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

    /// This tests how is generated type name in proxy
    #[endpoint]
    fn echo_managed_option(&self, mo: ManagedOption<BigUint>) -> ManagedOption<BigUint> {
        mo
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
        mv: ManagedVec<EsdtTokenIdentifier>,
    ) -> ManagedVec<EsdtTokenIdentifier> {
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

    #[endpoint]
    fn echo_varags_vec_with_counted(
        &self,
        m: MultiValueEncoded<MultiValue2<ManagedBuffer, MultiValueManagedVecCounted<usize>>>,
    ) -> MultiValueEncoded<MultiValue2<ManagedBuffer, MultiValueManagedVecCounted<usize>>> {
        m
    }

    #[endpoint]
    fn echo_varags_vec_with_counted_pairs(
        &self,
        m: MultiValueEncoded<
            MultiValue2<
                ManagedBuffer,
                MultiValueEncodedCounted<MultiValue2<usize, ManagedAddress>>,
            >,
        >,
    ) -> MultiValueEncoded<
        MultiValue2<ManagedBuffer, MultiValueEncodedCounted<MultiValue2<usize, ManagedAddress>>>,
    > {
        m
    }

    #[endpoint]
    fn convert_varags_vec_with_counted_pairs_1(
        &self,
        address_number_pairs: MultiValueEncoded<
            MultiValue3<ManagedAddress, usize, MultiValueEncodedCounted<MultiValue2<usize, usize>>>,
        >,
    ) -> MultiValueManagedVec<
        MultiValue3<ManagedAddress, usize, MultiValueManagedVecCounted<MultiValue2<usize, usize>>>,
    > {
        let mut result = MultiValueManagedVec::new();
        for triple in address_number_pairs {
            let (address, num, counted_lazy) = triple.into_tuple();
            let mut counted_list = MultiValueManagedVecCounted::new();
            for pair in counted_lazy {
                counted_list.push(pair);
            }
            result.push((address, num, counted_list).into());
        }
        result
    }

    #[endpoint]
    fn convert_varags_vec_with_counted_pairs_2(
        &self,
        address_number_pairs: MultiValueManagedVec<
            MultiValue3<
                ManagedAddress,
                usize,
                MultiValueManagedVecCounted<MultiValue2<usize, usize>>,
            >,
        >,
    ) -> MultiValueEncoded<
        MultiValue3<ManagedAddress, usize, MultiValueEncodedCounted<MultiValue2<usize, usize>>>,
    > {
        let mut result = MultiValueEncoded::new();
        for triple in address_number_pairs {
            let (address, x, counted_list) = triple.into_tuple();
            let mut counted_lazy = MultiValueEncodedCounted::new();
            let v = counted_list.into_vec();
            for pair in v {
                counted_lazy.push(pair);
            }
            result.push((address, x, counted_lazy).into());
        }
        result
    }
}
