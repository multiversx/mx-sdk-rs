use core::marker::PhantomData;

use elrond_codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    contract_base::ExitCodecErrorHandler,
    err_msg,
    types::{
        BigUint, EgldOrEsdtTokenPayment, EsdtTokenPayment, ManagedAddress, ManagedBuffer,
        ManagedVec,
    },
};

use super::{
    contract_call_common::UNSPECIFIED_GAS_LIMIT, contract_call_full::ContractCallFull,
    contract_call_with_egld::ContractCallWithEgld,
    contract_call_with_multi_esdt::ContractCallWithMultiEsdt, ContractCallTrait,
    ContractCallWithEgldOrSingleEsdt, ManagedArgBuffer,
};

/// Represents metadata for calling another contract, without payments.
#[must_use]
pub struct ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(super) _phantom: PhantomData<SA>,
    pub to: ManagedAddress<SA>,
    pub endpoint_name: ManagedBuffer<SA>,
    pub arg_buffer: ManagedArgBuffer<SA>,
    pub explicit_gas_limit: u64,
    pub(super) _return_type: PhantomData<OriginalResult>,
}

impl<SA, OriginalResult> ContractCallTrait<SA, OriginalResult>
    for ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    fn into_contract_call_full(self) -> ContractCallFull<SA, OriginalResult> {
        ContractCallFull {
            basic: self,
            egld_payment: BigUint::zero(),
            payments: ManagedVec::new(),
        }
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        self
    }
}

impl<SA, OriginalResult> ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub fn proxy_new(to: ManagedAddress<SA>, endpoint_name: &'static str) -> Self {
        Self::new(to, endpoint_name.into())
    }

    pub fn proxy_arg<T: TopEncodeMulti>(&mut self, endpoint_arg: &T) {
        super::contract_call_common::proxy_arg(&mut self.arg_buffer, endpoint_arg)
    }

    pub fn new(to: ManagedAddress<SA>, endpoint_name: ManagedBuffer<SA>) -> Self {
        ContractCallNoPayment {
            _phantom: PhantomData,
            to,
            endpoint_name,
            arg_buffer: ManagedArgBuffer::new(),
            explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
            _return_type: PhantomData,
        }
    }

    pub fn with_arguments_raw(mut self, raw_argument_buffer: ManagedArgBuffer<SA>) -> Self {
        self.arg_buffer = raw_argument_buffer;
        self
    }

    /// Provided for cases where we build the contract call by hand.
    pub fn push_arg_managed_buffer(&mut self, m_buffer: ManagedBuffer<SA>) {
        self.arg_buffer.push_arg_raw(m_buffer)
    }

    /// Provided for cases where we build the contract call by hand.
    /// Convenience method, also creates the new managed buffer from bytes.
    pub fn push_argument_raw_bytes(&mut self, bytes: &[u8]) {
        self.arg_buffer
            .push_arg_raw(ManagedBuffer::new_from_bytes(bytes));
    }

    pub fn push_endpoint_arg<T: TopEncodeMulti>(&mut self, endpoint_arg: &T) {
        let h = ExitCodecErrorHandler::<SA>::from(err_msg::CONTRACT_CALL_ENCODE_ERROR);
        let Ok(()) = endpoint_arg.multi_encode_or_handle_err(&mut self.arg_buffer, h);
    }

    /// Sets payment to be EGLD transfer.
    pub fn with_egld_transfer(
        self,
        egld_amount: BigUint<SA>,
    ) -> ContractCallWithEgld<SA, OriginalResult> {
        ContractCallWithEgld {
            basic: self,
            egld_payment: egld_amount,
        }
    }

    /// Adds a single ESDT token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    pub fn with_esdt_transfer<P: Into<EsdtTokenPayment<SA>>>(
        self,
        payment: P,
    ) -> ContractCallWithMultiEsdt<SA, OriginalResult> {
        let result = ContractCallWithMultiEsdt {
            basic: self,
            payments: ManagedVec::new(),
        };
        result.with_esdt_transfer(payment)
    }

    /// Sets payment to be a (potentially) multi-token transfer.
    #[inline]
    pub fn with_multi_token_transfer(
        self,
        payments: ManagedVec<SA, EsdtTokenPayment<SA>>,
    ) -> ContractCallWithMultiEsdt<SA, OriginalResult> {
        ContractCallWithMultiEsdt {
            basic: self,
            payments,
        }
    }

    /// Sets payment to be either EGLD or a single ESDT transfer, as determined at runtime.
    pub fn with_egld_or_single_esdt_transfer<P: Into<EgldOrEsdtTokenPayment<SA>>>(
        self,
        payment: P,
    ) -> ContractCallWithEgldOrSingleEsdt<SA, OriginalResult> {
        ContractCallWithEgldOrSingleEsdt {
            basic: self,
            payment: payment.into(),
        }
    }
}
