use elrond_codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{
        BigUint, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment, EsdtTokenPayment,
        ManagedAddress, ManagedVec,
    },
};

use super::{
    contract_call_full::ContractCallFull, contract_call_no_payment::ContractCallNoPayment,
    ContractCallTrait,
};

#[must_use]
pub struct ContractCallWithEgldOrSingleEsdt<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(super) basic: ContractCallNoPayment<SA, OriginalResult>,
    pub(super) payment: EgldOrEsdtTokenPayment<SA>,
}

impl<SA, OriginalResult> ContractCallTrait<SA>
    for ContractCallWithEgldOrSingleEsdt<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_contract_call_full(self) -> ContractCallFull<SA, OriginalResult> {
        if self.payment.token_identifier.is_egld() {
            ContractCallFull {
                basic: self.basic,
                egld_payment: self.payment.amount,
                payments: ManagedVec::new(),
            }
        } else {
            ContractCallFull {
                basic: self.basic,
                egld_payment: BigUint::zero(),
                payments: ManagedVec::from_single_item(EsdtTokenPayment::new(
                    self.payment.token_identifier.unwrap_esdt(),
                    self.payment.token_nonce,
                    self.payment.amount,
                )),
            }
        }
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
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

    pub fn proxy_arg<T: TopEncodeMulti>(&mut self, endpoint_arg: &T) {
        super::contract_call_common::proxy_arg(&mut self.basic.arg_buffer, endpoint_arg)
    }
}
