use elrond_codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{BigUint, ManagedAddress, ManagedVec},
};

use super::{
    contract_call_full::ContractCallFull, contract_call_no_payment::ContractCallNoPayment,
    ContractCall,
};

#[must_use]
pub struct ContractCallWithEgld<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(super) basic: ContractCallNoPayment<SA, OriginalResult>,
    pub(super) egld_payment: BigUint<SA>,
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithEgld<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_contract_call_full(self) -> ContractCallFull<SA, OriginalResult> {
        ContractCallFull {
            basic: self.basic,
            egld_payment: self.egld_payment,
            payments: ManagedVec::new(),
        }
    }

    #[inline]
    fn into_contract_call_normalized(self) -> ContractCallFull<SA, Self::OriginalResult> {
        // no ESDT, no conversion needed
        self.into_contract_call_full()
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }
}

impl<SA, OriginalResult> ContractCallWithEgld<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub fn proxy_new(
        to: ManagedAddress<SA>,
        endpoint_name: &'static str,
        egld_payment: BigUint<SA>,
    ) -> Self {
        ContractCallWithEgld {
            basic: ContractCallNoPayment::proxy_new(to, endpoint_name),
            egld_payment,
        }
    }
}
