use alloc::string::String;

use crate::{
    abi::TypeAbi,
    api::{EndpointFinishApi, ManagedTypeApi},
    types::{BigUint, ManagedVecItem},
    ArgId, ContractCallArg, DynArg, DynArgInput, DynArgOutput, EndpointResult,
};

use super::{EsdtTokenPayment, TokenIdentifier};

/// Thin wrapper around EsdtTokenPayment, which has different I/O behaviour:
/// - as input, is built from 3 arguments instead of 1: token identifier, nonce, value
/// - as output, it becomes 3 results instead of 1: token identifier, nonce, value
#[derive(Clone, PartialEq, Debug)]
pub struct EsdtTokenPaymentMultiArg<M: ManagedTypeApi> {
    obj: EsdtTokenPayment<M>,
}

impl<M: ManagedTypeApi> From<EsdtTokenPayment<M>> for EsdtTokenPaymentMultiArg<M> {
    #[inline]
    fn from(obj: EsdtTokenPayment<M>) -> Self {
        EsdtTokenPaymentMultiArg { obj }
    }
}

impl<M: ManagedTypeApi> EsdtTokenPaymentMultiArg<M> {
    pub fn into_esdt_token_payment(self) -> EsdtTokenPayment<M> {
        self.obj
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for EsdtTokenPaymentMultiArg<M> {
    const PAYLOAD_SIZE: usize = EsdtTokenPayment::<M>::PAYLOAD_SIZE;
    const SKIPS_RESERIALIZATION: bool = EsdtTokenPayment::<M>::SKIPS_RESERIALIZATION;

    #[inline]
    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
        EsdtTokenPayment::from_byte_reader(reader).into()
    }

    #[inline]
    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
        self.obj.to_byte_writer(writer)
    }
}

impl<M> DynArg for EsdtTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
{
    fn dyn_load<I: DynArgInput>(loader: &mut I, arg_id: ArgId) -> Self {
        let token_identifier = TokenIdentifier::dyn_load(loader, arg_id);
        let token_nonce = u64::dyn_load(loader, arg_id);
        let amount = BigUint::dyn_load(loader, arg_id);
        EsdtTokenPayment::new(token_identifier, token_nonce, amount).into()
    }
}

impl<M> EndpointResult for EsdtTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
{
    type DecodeAs = EsdtTokenPaymentMultiArg<M>;

    #[inline]
    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        self.obj.token_identifier.finish(api.clone());
        self.obj.token_nonce.finish(api.clone());
        self.obj.amount.finish(api);
    }
}

impl<M> ContractCallArg for &EsdtTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        self.obj.token_identifier.push_dyn_arg(output);
        self.obj.token_nonce.push_dyn_arg(output);
        self.obj.amount.push_dyn_arg(output);
    }
}

impl<M> ContractCallArg for EsdtTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        (&self).push_dyn_arg(output)
    }
}

impl<M> TypeAbi for EsdtTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
{
    fn type_name() -> String {
        crate::types::MultiArg3::<TokenIdentifier<M>, u64, BigUint<M>>::type_name()
    }

    fn is_multi_arg_or_result() -> bool {
        true
    }
}
