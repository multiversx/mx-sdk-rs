use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{
        BigUint, EsdtTokenPayment, ManagedAddress, ManagedBuffer, ManagedVec, TokenIdentifier,
    },
};

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCall, ContractCallWithEgld};

#[must_use]
pub struct ContractCallWithMultiEsdt<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub basic: ContractCallNoPayment<SA, OriginalResult>,
    pub esdt_payments: ManagedVec<SA, EsdtTokenPayment<SA>>,
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithMultiEsdt<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_normalized(self) -> ContractCallWithEgld<SA, Self::OriginalResult> {
        self.basic
            .into_normalized()
            .convert_to_esdt_transfer_call(self.esdt_payments)
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }

    fn transfer_execute(self) {
        self.basic.transfer_execute_esdt(self.esdt_payments);
    }
}

impl<SA, OriginalResult> ContractCallWithMultiEsdt<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    /// Creates a new instance directly.
    ///
    /// The constructor is mostly for hand-written proxies,
    /// the usual way of constructing this object is via the builder methods of other contract call types,
    /// especially `with_esdt_transfer` or `with_multi_token_transfer`.
    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        payments: ManagedVec<SA, EsdtTokenPayment<SA>>,
    ) -> Self {
        ContractCallWithMultiEsdt {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            esdt_payments: payments,
        }
    }

    /// Adds a single ESDT token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    pub fn with_esdt_transfer<P: Into<EsdtTokenPayment<SA>>>(mut self, payment: P) -> Self {
        self.esdt_payments.push(payment.into());
        self
    }

    #[deprecated(
        since = "0.39.0",
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
