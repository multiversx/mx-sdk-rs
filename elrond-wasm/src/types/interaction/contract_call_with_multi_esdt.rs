use elrond_codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{BigUint, EsdtTokenPayment, ManagedAddress, ManagedVec},
};

use super::{
    contract_call_full::ContractCallFull, contract_call_no_payment::ContractCallNoPayment,
    ContractCallTrait,
};

#[must_use]
pub struct ContractCallWithMultiEsdt<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(super) basic: ContractCallNoPayment<SA, OriginalResult>,
    pub(super) payments: ManagedVec<SA, EsdtTokenPayment<SA>>,
}

impl<SA, OriginalResult> ContractCallTrait<SA> for ContractCallWithMultiEsdt<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_contract_call_full(self) -> ContractCallFull<SA, OriginalResult> {
        ContractCallFull {
            basic: self.basic,
            egld_payment: BigUint::zero(),
            payments: self.payments,
        }
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }
}

impl<SA, OriginalResult> ContractCallWithMultiEsdt<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub fn proxy_new(
        to: ManagedAddress<SA>,
        endpoint_name: &'static str,
        payments: ManagedVec<SA, EsdtTokenPayment<SA>>,
    ) -> Self {
        ContractCallWithMultiEsdt {
            basic: ContractCallNoPayment::proxy_new(to, endpoint_name),
            payments,
        }
    }

    /// Adds a single ESDT token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    pub fn with_esdt_transfer<P: Into<EsdtTokenPayment<SA>>>(mut self, payment: P) -> Self {
        self.payments.push(payment.into());
        self
    }
}
