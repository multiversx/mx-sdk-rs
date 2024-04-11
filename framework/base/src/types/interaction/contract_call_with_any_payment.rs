use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{EgldOrMultiEsdtPayment, ManagedAddress, ManagedBuffer},
};

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCall, ContractCallWithEgld};

/// Holds data for calling another contract, with any type of payment: none, EGLD, Multi-ESDT.
///
/// Gets created when chaining method `with_any_payment`.
#[must_use]
pub struct ContractCallWithAnyPayment<'a, SA, OriginalResult>
where
    SA: CallTypeApi<'a> + 'static,
{
    pub basic: ContractCallNoPayment<'a, SA, OriginalResult>,
    pub payment: EgldOrMultiEsdtPayment<'a, SA>,
}

impl<'a, SA, OriginalResult> ContractCall<'a, SA> for ContractCallWithAnyPayment<'a, SA, OriginalResult>
where
    SA: CallTypeApi<'a> + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_normalized(self) -> ContractCallWithEgld<'a, SA, Self::OriginalResult> {
        match self.payment {
            EgldOrMultiEsdtPayment::Egld(egld_amount) => self.basic.with_egld_transfer(egld_amount),
            EgldOrMultiEsdtPayment::MultiEsdt(multi_esdt_payment) => self
                .basic
                .into_normalized()
                .convert_to_esdt_transfer_call(multi_esdt_payment),
        }
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<'a, SA, OriginalResult> {
        &mut self.basic
    }

    fn transfer_execute(self) {
        match self.payment {
            EgldOrMultiEsdtPayment::Egld(egld_amount) => {
                self.basic.transfer_execute_egld(egld_amount);
            },
            EgldOrMultiEsdtPayment::MultiEsdt(multi_esdt_payment) => {
                self.basic.transfer_execute_esdt(multi_esdt_payment);
            },
        }
    }
}

impl<'a, SA, OriginalResult> ContractCallWithAnyPayment<'a, SA, OriginalResult>
where
    SA: CallTypeApi<'a> + 'static,
{
    /// Creates a new instance directly.
    pub fn new<N: Into<ManagedBuffer<'a, SA>>>(
        to: ManagedAddress<'a, SA>,
        endpoint_name: N,
        payment: EgldOrMultiEsdtPayment<'a, SA>,
    ) -> Self {
        ContractCallWithAnyPayment {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            payment,
        }
    }
}
