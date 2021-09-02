use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::SendApi;
use crate::io::EndpointResult;
use crate::types::{BigUint, ManagedAddress, ManagedBuffer};
use alloc::string::String;
use alloc::vec::Vec;

pub struct SendEgld<SA>
where
    SA: SendApi + 'static,
{
    pub api: SA,
    pub to: ManagedAddress<SA::ProxyTypeManager>,
    pub amount: BigUint<SA::ProxyTypeManager>,
    pub data: ManagedBuffer<SA::ProxyTypeManager>,
}

impl<SA> EndpointResult for SendEgld<SA>
where
    SA: SendApi + 'static,
{
    type DecodeAs = ();

    #[inline]
    fn finish<FA>(&self, _api: FA) {
        self.api.direct_egld(&self.to, &self.amount, &self.data);
    }
}

impl<SA> TypeAbi for SendEgld<SA>
where
    SA: SendApi + 'static,
{
    fn type_name() -> String {
        "SendEgld".into()
    }

    /// No ABI output.
    fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
        Vec::new()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
