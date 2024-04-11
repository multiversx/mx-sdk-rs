use core::marker::PhantomData;

use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{
        BigUint, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment, EgldOrMultiEsdtPayment,
        EsdtTokenPayment, ManagedAddress, ManagedBuffer, ManagedVec, TokenIdentifier,
    },
};

use super::{
    contract_call_exec::UNSPECIFIED_GAS_LIMIT, contract_call_with_egld::ContractCallWithEgld,
    contract_call_with_multi_esdt::ContractCallWithMultiEsdt, ContractCall,
    ContractCallWithAnyPayment, ContractCallWithEgldOrSingleEsdt, FunctionCall, ManagedArgBuffer,
};

/// Holds metadata for calling another contract, without payments.
///
/// Proxies generally create contract calls of this type
/// (unless there are payment arguments in the endpoint - but these are mostly obsolete now).
///
/// It is also the basis for all other contract call types, all of them contain this one.
#[must_use]
pub struct ContractCallNoPayment<'a, SA, OriginalResult>
where
    SA: CallTypeApi<'a>,
{
    pub(super) _phantom: PhantomData<SA>,
    pub to: ManagedAddress<'a, SA>,
    pub function_call: FunctionCall<'a, SA>,
    pub explicit_gas_limit: u64,
    pub(super) _return_type: PhantomData<OriginalResult>,
}

impl<'a, SA, OriginalResult> ContractCall<'a, SA> for ContractCallNoPayment<'a, SA, OriginalResult>
where
    SA: CallTypeApi<'a> + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    #[inline]
    fn into_normalized(self) -> ContractCallWithEgld<'a, SA, Self::OriginalResult> {
        ContractCallWithEgld {
            basic: self,
            egld_payment: BigUint::zero(),
        }
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<'a, SA, OriginalResult> {
        self
    }

    fn transfer_execute(self) {
        self.transfer_execute_egld(BigUint::zero());
    }
}

impl<'a, SA, OriginalResult> ContractCallNoPayment<'a, SA, OriginalResult>
where
    SA: CallTypeApi<'a> + 'static,
{
    pub fn new<N: Into<ManagedBuffer<'a, SA>>>(to: ManagedAddress<'a, SA>, endpoint_name: N) -> Self {
        ContractCallNoPayment {
            _phantom: PhantomData,
            to,
            function_call: FunctionCall {
                function_name: endpoint_name.into(),
                arg_buffer: ManagedArgBuffer::new(),
            },
            explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
            _return_type: PhantomData,
        }
    }

    /// Sets payment to be EGLD transfer.
    pub fn with_egld_transfer(
        self,
        egld_amount: BigUint<'a, SA>,
    ) -> ContractCallWithEgld<'a, SA, OriginalResult> {
        ContractCallWithEgld {
            basic: self,
            egld_payment: egld_amount,
        }
    }

    /// Adds a single ESDT token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    pub fn with_esdt_transfer<P: Into<EsdtTokenPayment<'a, SA>>>(
        self,
        payment: P,
    ) -> ContractCallWithMultiEsdt<'a, SA, OriginalResult> {
        let result = ContractCallWithMultiEsdt {
            basic: self,
            esdt_payments: ManagedVec::new(),
        };
        result.with_esdt_transfer(payment)
    }

    #[deprecated(
        since = "0.39.0",
        note = "Replace by `contract_call.with_esdt_transfer((payment_token, payment_nonce, payment_amount))`. 
        The tuple argument will get automatically converted to EsdtTokenPayment."
    )]
    pub fn add_esdt_token_transfer(
        self,
        payment_token: TokenIdentifier<'a, SA>,
        payment_nonce: u64,
        payment_amount: BigUint<'a, SA>,
    ) -> ContractCallWithMultiEsdt<'a, SA, OriginalResult> {
        self.with_esdt_transfer((payment_token, payment_nonce, payment_amount))
    }

    /// Sets payment to be a (potentially) multi-token transfer.
    #[inline]
    pub fn with_multi_token_transfer(
        self,
        payments: ManagedVec<'a, SA, EsdtTokenPayment<'a, SA>>,
    ) -> ContractCallWithMultiEsdt<'a, SA, OriginalResult> {
        ContractCallWithMultiEsdt {
            basic: self,
            esdt_payments: payments,
        }
    }

    /// Sets payment to be a (potentially) multi-token transfer.
    #[inline]
    pub fn with_any_payment(
        self,
        payment: EgldOrMultiEsdtPayment<'a, SA>,
    ) -> ContractCallWithAnyPayment<'a, SA, OriginalResult> {
        ContractCallWithAnyPayment {
            basic: self,
            payment,
        }
    }

    /// Sets payment to be either EGLD or a single ESDT transfer, as determined at runtime.
    pub fn with_egld_or_single_esdt_transfer<P: Into<EgldOrEsdtTokenPayment<'a, SA>>>(
        self,
        payment: P,
    ) -> ContractCallWithEgldOrSingleEsdt<'a, SA, OriginalResult> {
        ContractCallWithEgldOrSingleEsdt {
            basic: self,
            payment: payment.into(),
        }
    }

    #[deprecated(
        since = "0.39.0",
        note = "Replace by `contract_call.with_egld_or_single_esdt_transfer((payment_token, payment_nonce, payment_amount))`. "
    )]
    pub fn with_egld_or_single_esdt_token_transfer(
        self,
        payment_token: EgldOrEsdtTokenIdentifier<'a, SA>,
        payment_nonce: u64,
        payment_amount: BigUint<'a, SA>,
    ) -> ContractCallWithEgldOrSingleEsdt<'a, SA, OriginalResult> {
        self.with_egld_or_single_esdt_transfer((payment_token, payment_nonce, payment_amount))
    }

    pub fn into_function_call(self) -> FunctionCall<'a, SA> {
        self.function_call
    }
}
