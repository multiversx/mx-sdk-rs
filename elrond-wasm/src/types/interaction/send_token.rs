use crate::{
    abi::{OutputAbi, TypeAbi, TypeDescriptionContainer},
    api::{SendApi, StorageReadApi},
    contract_base::SendWrapper,
    io::EndpointResult,
    types::{BigUint, ManagedAddress, ManagedBuffer, TokenIdentifier},
};
use alloc::{string::String, vec::Vec};

pub struct SendToken<SA>
where
    SA: SendApi + StorageReadApi + 'static,
{
    pub api: SA,
    pub to: ManagedAddress<SA>,
    pub token: TokenIdentifier<SA>,
    pub amount: BigUint<SA>,
    pub data: ManagedBuffer<SA>,
}

impl<SA> EndpointResult for SendToken<SA>
where
    SA: SendApi + StorageReadApi + 'static,
{
    type DecodeAs = ();

    #[inline]
    fn finish<FA>(&self, _api: FA) {
        if self.token.is_egld() {
            self.api
                .direct_egld(&self.to, &self.amount, self.data.clone());
        } else {
            SendWrapper::new(self.api.clone()).transfer_esdt_via_async_call(
                &self.to,
                &self.token,
                0,
                &self.amount,
                self.data.clone(),
            );
        }
    }
}

impl<SA> TypeAbi for SendToken<SA>
where
    SA: SendApi + StorageReadApi + 'static,
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
