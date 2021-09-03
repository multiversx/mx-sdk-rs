use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::SendApi;
use crate::io::EndpointResult;
use crate::types::{BigUint, ManagedAddress, ManagedBuffer, TokenIdentifier};
use alloc::string::String;
use alloc::vec::Vec;

pub struct SendEsdt<SA>
where
    SA: SendApi + 'static,
{
    pub(super) api: SA,
    pub(super) to: ManagedAddress<SA::ProxyTypeManager>,
    pub(super) token_identifier: TokenIdentifier<SA::ProxyTypeManager>,
    pub(super) amount: BigUint<SA::ProxyTypeManager>,
    pub data: ManagedBuffer<SA::ProxyTypeManager>,
}

impl<SA> EndpointResult for SendEsdt<SA>
where
    SA: SendApi + 'static,
{
    type DecodeAs = ();

    #[inline]
    fn finish<FA>(&self, _api: FA) {
        self.api.transfer_esdt_via_async_call(
            &self.to,
            &self.token_identifier,
            0,
            &self.amount,
            self.data.clone(),
        );
    }
}

impl<SA> TypeAbi for SendEsdt<SA>
where
    SA: SendApi + 'static,
{
    fn type_name() -> String {
        "SendEsdt".into()
    }

    /// No ABI output.
    fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
        Vec::new()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
