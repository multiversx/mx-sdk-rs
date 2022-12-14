use elrond_codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{BigUint, ManagedAddress, ManagedBuffer},
};

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCall};

#[must_use]
pub struct ContractCallWithEgld<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub basic: ContractCallNoPayment<SA, OriginalResult>,
    pub egld_payment: BigUint<SA>,
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithEgld<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    #[inline]
    fn into_normalized(self) -> ContractCallWithEgld<SA, Self::OriginalResult> {
        // no ESDT, no conversion needed
        self
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }

    fn transfer_execute(self) {
        self.basic.transfer_execute_egld(self.egld_payment);
    }
}

impl<SA, OriginalResult> ContractCallWithEgld<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        egld_payment: BigUint<SA>,
    ) -> Self {
        ContractCallWithEgld {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            egld_payment,
        }
    }

    pub fn proxy_new(
        to: ManagedAddress<SA>,
        endpoint_name: &'static str,
        egld_payment: BigUint<SA>,
    ) -> Self {
        ContractCallWithEgld::new(to, endpoint_name, egld_payment)
    }
}
