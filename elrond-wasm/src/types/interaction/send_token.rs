use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::SendApi;
use crate::io::EndpointResult;
use crate::types::{Address, BigUint, BoxedBytes, TokenIdentifier};
use alloc::string::String;
use alloc::vec::Vec;

pub struct SendToken<SA>
where
    SA: SendApi + 'static,
{
    pub api: SA,
    pub to: Address,
    pub token: TokenIdentifier<SA::ProxyTypeManager>,
    pub amount: BigUint<SA::ProxyTypeManager>,
    pub data: BoxedBytes,
}

impl<SA> EndpointResult for SendToken<SA>
where
    SA: SendApi + 'static,
{
    type DecodeAs = ();

    #[inline]
    fn finish<FA>(&self, _api: FA) {
        if self.token.is_egld() {
            self.api
                .direct_egld(&self.to, &self.amount, self.data.as_slice());
        } else {
            self.api.transfer_esdt_via_async_call(
                &self.to,
                &self.token,
                0,
                &self.amount,
                self.data.as_slice(),
            );
        }
    }
}

impl<SA> TypeAbi for SendToken<SA>
where
    SA: SendApi + 'static,
{
    fn type_name() -> String {
        "SendToken".into()
    }

    /// No ABI output.
    fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
        Vec::new()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
