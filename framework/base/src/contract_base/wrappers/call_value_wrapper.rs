use core::marker::PhantomData;

use crate::{
    api::{
        const_handles, use_raw_handle, CallValueApi, CallValueApiImpl, ErrorApi, ErrorApiImpl,
        HandleConstraints, ManagedTypeApi, StaticVarApiImpl,
    },
    err_msg,
    types::{
        BigUint, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment, EgldOrMultiEsdtPayment,
        EsdtTokenPayment, ManagedRef, ManagedVec, TokenIdentifier,
    },
};

#[derive(Default)]
pub struct CallValueWrapper<A>
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    _phantom: PhantomData<A>,
}

impl<A> CallValueWrapper<A>
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    pub fn new() -> Self {
        CallValueWrapper {
            _phantom: PhantomData,
        }
    }

    /// Retrieves the EGLD call value from the VM.
    /// Will return 0 in case of an ESDT transfer (cannot have both EGLD and ESDT transfer simultaneously).
    pub fn egld_value(&self) -> ManagedRef<'static, A, BigUint<A>> {
        let mut call_value_handle: A::BigIntHandle =
            use_raw_handle(A::static_var_api_impl().get_call_value_egld_handle());
        if call_value_handle == const_handles::UNINITIALIZED_HANDLE {
            call_value_handle = use_raw_handle(const_handles::CALL_VALUE_EGLD);
            A::static_var_api_impl().set_call_value_egld_handle(call_value_handle.get_raw_handle());
            A::call_value_api_impl().load_egld_value(call_value_handle.clone());
        }
        unsafe { ManagedRef::wrap_handle(call_value_handle) }
    }

    /// Returns all ESDT transfers that accompany this SC call.
    /// Will return 0 results if nothing was transfered, or just EGLD.
    /// Fully managed underlying types, very efficient.
    pub fn all_esdt_transfers(&self) -> ManagedRef<'static, A, ManagedVec<A, EsdtTokenPayment<A>>> {
        let mut call_value_handle: A::ManagedBufferHandle =
            use_raw_handle(A::static_var_api_impl().get_call_value_multi_esdt_handle());
        if call_value_handle == const_handles::UNINITIALIZED_HANDLE {
            call_value_handle = use_raw_handle(const_handles::CALL_VALUE_MULTI_ESDT);
            A::static_var_api_impl()
                .set_call_value_multi_esdt_handle(call_value_handle.get_raw_handle());
            A::call_value_api_impl().load_all_esdt_transfers(call_value_handle.clone());
        }
        unsafe { ManagedRef::wrap_handle(call_value_handle) }
    }

    /// Verify and casts the received multi ESDT transfer in to an array.
    ///
    /// Can be used to extract all payments in one line like this:
    ///
    /// `let [payment_a, payment_b, payment_c] = self.call_value().multi_esdt();`.
    pub fn multi_esdt<const N: usize>(&self) -> [EsdtTokenPayment<A>; N] {
        self.all_esdt_transfers()
            .to_array_of_refs::<N>()
            .unwrap_or_else(|| {
                A::error_api_impl().signal_error(err_msg::INCORRECT_NUM_ESDT_TRANSFERS.as_bytes())
            })
    }

    /// Expects precisely one ESDT token transfer, fungible or not.
    ///
    /// Will return the received ESDT payment.
    ///
    /// The amount cannot be 0, since that would not qualify as an ESDT transfer.
    pub fn single_esdt(&self) -> EsdtTokenPayment<A> {
        let [payments] = self.multi_esdt();
        payments
    }

    /// Expects precisely one fungible ESDT token transfer.
    ///
    /// Returns the token ID and the amount for fungible ESDT transfers.
    ///
    /// The amount cannot be 0, since that would not qualify as an ESDT transfer.
    pub fn single_fungible_esdt(&self) -> (TokenIdentifier<A>, BigUint<A>) {
        let payment = self.single_esdt();
        if payment.token_nonce != 0 {
            A::error_api_impl().signal_error(err_msg::FUNGIBLE_TOKEN_EXPECTED_ERR_MSG.as_bytes());
        }
        (payment.token_identifier, payment.amount)
    }

    /// Accepts and returns either an EGLD payment, or a single ESDT token.
    ///
    /// Will halt execution if more than one ESDT transfer was received.
    ///
    /// In case no transfer of value happen, it will return a payment of 0 EGLD.
    pub fn egld_or_single_esdt(&self) -> EgldOrEsdtTokenPayment<A> {
        let esdt_transfers = self.all_esdt_transfers();
        match esdt_transfers.len() {
            0 => EgldOrEsdtTokenPayment {
                token_identifier: EgldOrEsdtTokenIdentifier::egld(),
                token_nonce: 0,
                amount: self.egld_value().clone_value(),
            },
            1 => esdt_transfers.get(0).into(),
            _ => A::error_api_impl().signal_error(err_msg::INCORRECT_NUM_ESDT_TRANSFERS.as_bytes()),
        }
    }

    /// Accepts and returns either an EGLD payment, or a single fungible ESDT token.
    ///
    /// Will halt execution if more than one ESDT transfer was received, or if the received ESDT is non- or semi-fungible.
    ///
    /// Works similar to `egld_or_single_esdt`,
    /// but checks the nonce to be 0 and returns a tuple of just token identifier and amount, for convenience.
    ///
    /// In case no transfer of value happen, it will return a payment of 0 EGLD.
    pub fn egld_or_single_fungible_esdt(&self) -> (EgldOrEsdtTokenIdentifier<A>, BigUint<A>) {
        let payment = self.egld_or_single_esdt();
        if payment.token_nonce != 0 {
            A::error_api_impl().signal_error(err_msg::FUNGIBLE_TOKEN_EXPECTED_ERR_MSG.as_bytes());
        }

        (payment.token_identifier, payment.amount)
    }

    /// Accepts any sort of patyment, which is either:
    /// - EGLD (can be zero in case of no payment whatsoever);
    /// - Multi-ESDT (one or more ESDT transfers).
    pub fn any_payment(&self) -> EgldOrMultiEsdtPayment<A> {
        let esdt_transfers = self.all_esdt_transfers();
        if esdt_transfers.is_empty() {
            EgldOrMultiEsdtPayment::Egld(self.egld_value().clone_value())
        } else {
            EgldOrMultiEsdtPayment::MultiEsdt(esdt_transfers.clone_value())
        }
    }
}
