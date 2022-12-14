use elrond_codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{
        BigUint, EsdtTokenPayment, ManagedAddress, ManagedBuffer, ManagedVec, TokenIdentifier,
    },
};

use super::{
    contract_call_full::ContractCallFull, contract_call_no_payment::ContractCallNoPayment,
    ContractCall,
};

#[must_use]
pub struct ContractCallWithMultiEsdt<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub basic: ContractCallNoPayment<SA, OriginalResult>,
    pub payments: ManagedVec<SA, EsdtTokenPayment<SA>>,
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithMultiEsdt<SA, OriginalResult>
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
        ContractCallWithMultiEsdt::new(to, endpoint_name, payments)
    }

    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        payments: ManagedVec<SA, EsdtTokenPayment<SA>>,
    ) -> Self {
        ContractCallWithMultiEsdt {
            basic: ContractCallNoPayment::new(to, endpoint_name),
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

    #[deprecated(
        since = "0.38.0",
        note = "Replace by `contract_call.with_esdt_transfer((payment_token, payment_nonce, payment_amount))`. 
        The tuple argument will get automatically converted to EsdtTokenPayment."
    )]
    pub fn add_esdt_token_transfer(
        self,
        payment_token: TokenIdentifier<SA>,
        payment_nonce: u64,
        payment_amount: BigUint<SA>,
    ) -> Self {
        self.with_esdt_transfer((payment_token, payment_nonce, payment_amount))
    }
}
