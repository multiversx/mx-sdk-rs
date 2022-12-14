use elrond_codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{BigUint, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment, ManagedAddress},
};

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCall, ContractCallWithEgld};

#[must_use]
pub struct ContractCallWithEgldOrSingleEsdt<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(super) basic: ContractCallNoPayment<SA, OriginalResult>,
    pub(super) payment: EgldOrEsdtTokenPayment<SA>,
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
    pub fn proxy_new(
        to: ManagedAddress<SA>,
        endpoint_name: &'static str,
        token_identifier: EgldOrEsdtTokenIdentifier<SA>,
        token_nonce: u64,
        amount: BigUint<SA>,
    ) -> Self {
        ContractCallWithEgldOrSingleEsdt {
            basic: ContractCallNoPayment::proxy_new(to, endpoint_name),
            payment: EgldOrEsdtTokenPayment::new(token_identifier, token_nonce, amount),
        }
    }
}
