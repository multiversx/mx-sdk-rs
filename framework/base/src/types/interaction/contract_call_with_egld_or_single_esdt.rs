use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{
        BigUint, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment, ManagedAddress, ManagedBuffer,
    },
};

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCall, ContractCallWithEgld};

/// Holds data for calling another contract, with a single payment, either EGLD or a single ESDT token.
///
/// Gets created when chaining method `with_egld_or_single_esdt_transfer`.
#[must_use]
pub struct ContractCallWithEgldOrSingleEsdt<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(super) basic: ContractCallNoPayment<SA, OriginalResult>,
    pub payment: EgldOrEsdtTokenPayment<SA>,
}

impl<SA, OriginalResult> ContractCallWithEgldOrSingleEsdt<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    fn into_normalized_egld(self) -> ContractCallWithEgld<SA, OriginalResult> {
        ContractCallWithEgld {
            basic: self.basic,
            egld_payment: self.payment.amount,
        }
    }

    fn into_normalized_esdt(self) -> ContractCallWithEgld<SA, OriginalResult> {
        self.basic
            .into_normalized()
            .convert_to_single_transfer_esdt_call(self.payment.unwrap_esdt())
    }
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithEgldOrSingleEsdt<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_normalized(self) -> ContractCallWithEgld<SA, Self::OriginalResult> {
        if self.payment.token_identifier.is_egld() {
            self.into_normalized_egld()
        } else {
            // Because we know that there can be at most one ESDT payment,
            // there is no need to call the full `convert_to_esdt_transfer_call`.
            self.into_normalized_esdt()
        }
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }

    fn transfer_execute(self) {
        if self.payment.token_identifier.is_egld() {
            self.basic.transfer_execute_egld(self.payment.amount);
        } else {
            self.basic
                .transfer_execute_single_esdt(self.payment.unwrap_esdt());
        }
    }
}

impl<SA, OriginalResult> ContractCallWithEgldOrSingleEsdt<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    /// Creates a new instance directly.
    ///
    /// The constructor is mostly for hand-written proxies,
    /// the usual way of constructing this object is via the builder methods of other contract call types,
    /// especially `with_egld_or_single_esdt_transfer`.
    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        token_identifier: EgldOrEsdtTokenIdentifier<SA>,
        token_nonce: u64,
        amount: BigUint<SA>,
    ) -> Self {
        ContractCallWithEgldOrSingleEsdt {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            payment: EgldOrEsdtTokenPayment::new(token_identifier, token_nonce, amount),
        }
    }
}
