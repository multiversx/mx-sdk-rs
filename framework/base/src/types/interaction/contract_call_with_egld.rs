use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{BigUint, ManagedAddress, ManagedBuffer},
};

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCall};

/// Holds data for calling another contract, with EGLD payment only.
///
/// Gets created when chaining method `with_egld_transfer`.
///
/// If the payment is zero, it bevahes exactly like `ContractCallNoPayment`.
///
/// It also represents the normalized form of any contract call, since ESDT transfers
/// (the only payment not available here) get converted to builtin function calls in normalized form.
#[must_use]
pub struct ContractCallWithEgld<'a, SA, OriginalResult>
where
    SA: CallTypeApi<'a> + 'static,
{
    pub basic: ContractCallNoPayment<'a, SA, OriginalResult>,
    pub egld_payment: BigUint<'a, SA>,
}

impl<'a, SA, OriginalResult> ContractCall<'a, SA> for ContractCallWithEgld<'a, SA, OriginalResult>
where
    SA: CallTypeApi<'a> + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    #[inline]
    fn into_normalized(self) -> ContractCallWithEgld<'a, SA, Self::OriginalResult> {
        // no ESDT, no conversion needed
        self
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<'a, SA, OriginalResult> {
        &mut self.basic
    }

    fn transfer_execute(self) {
        self.basic.transfer_execute_egld(self.egld_payment);
    }
}

impl<'a, SA, OriginalResult> ContractCallWithEgld<'a, SA, OriginalResult>
where
    SA: CallTypeApi<'a> + 'static,
{
    /// Creates a new instance directly.
    ///
    /// The constructor is mostly for hand-written proxies,
    /// the usual way of constructing this object is via the builder methods of other contract call types,
    /// especially `with_egld_transfer`.
    pub fn new<N: Into<ManagedBuffer<'a, SA>>>(
        to: ManagedAddress<'a, SA>,
        endpoint_name: N,
        egld_payment: BigUint<'a, SA>,
    ) -> Self {
        ContractCallWithEgld {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            egld_payment,
        }
    }
}
