use core::marker::PhantomData;

use multiversx_chain_core::EGLD_000000_TOKEN_IDENTIFIER;

use crate::{
    api::{
        const_handles, use_raw_handle, CallValueApi, CallValueApiImpl, ErrorApi, ErrorApiImpl,
        ManagedBufferApiImpl, ManagedTypeApi, RawHandle, StaticVarApiFlags, StaticVarApiImpl,
    },
    err_msg,
    types::{
        BigUint, EgldDecimals, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment,
        EgldOrMultiEsdtPayment, EsdtTokenPayment, ManagedDecimal, ManagedRef, ManagedType,
        ManagedVec, ManagedVecItem, ManagedVecItemPayload, ManagedVecPayloadIterator,
        ManagedVecRef, TokenIdentifier,
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

    /// Cached transfers from the VM.
    fn all_esdt_transfers_unchecked(&self) -> A::ManagedBufferHandle {
        let all_transfers_unchecked_handle: A::ManagedBufferHandle =
            use_raw_handle(const_handles::CALL_VALUE_MULTI_ESDT);
        if !A::static_var_api_impl()
            .flag_is_set_or_update(StaticVarApiFlags::CALL_VALUE_ESDT_UNCHECKED_INITIALIZED)
        {
            A::call_value_api_impl()
                .load_all_esdt_transfers(all_transfers_unchecked_handle.clone());
        }
        all_transfers_unchecked_handle
    }

    /// Retrieves the EGLD call value from the VM.
    ///
    /// Will return 0 in case of an ESDT transfer, even though EGLD and ESDT transfers are now possible.
    pub fn egld_direct_non_strict(&self) -> ManagedRef<'static, A, BigUint<A>> {
        let call_value_handle: A::BigIntHandle = use_raw_handle(const_handles::CALL_VALUE_EGLD);
        if !A::static_var_api_impl()
            .flag_is_set_or_update(StaticVarApiFlags::CALL_VALUE_EGLD_DIRECT_INITIALIZED)
        {
            A::call_value_api_impl().load_egld_value(call_value_handle.clone());
        }
        unsafe { ManagedRef::wrap_handle(call_value_handle) }
    }

    /// Retrieves the EGLD call value and crashes if anything else was transferred.
    ///
    /// Accepts both EGLD sent directly, as well as EGLD sent alone in a multi-transfer.
    ///
    /// Does not accept a multi-transfer with 2 or more transfers, not even 2 or more EGLD transfers.
    pub fn egld(&self) -> ManagedRef<'static, A, BigUint<A>> {
        let esdt_transfers_handle = self.all_esdt_transfers_unchecked();
        let esdt_transfers: ManagedRef<'static, A, ManagedVec<A, EgldOrEsdtTokenPayment<A>>> =
            unsafe { ManagedRef::wrap_handle(esdt_transfers_handle) };
        match esdt_transfers.len() {
            0 => self.egld_direct_non_strict(),
            1 => {
                let first = esdt_transfers.get(0);
                if !first.token_identifier.is_egld() {
                    A::error_api_impl().signal_error(err_msg::NON_PAYABLE_FUNC_ESDT.as_bytes());
                }
                unsafe { ManagedRef::wrap_handle(first.amount.get_handle()) }
            },
            _ => A::error_api_impl().signal_error(err_msg::INCORRECT_NUM_TRANSFERS.as_bytes()),
        }
    }

    /// Retrieves the EGLD call value from the VM.
    ///
    /// Will return 0 in case of an ESDT transfer, even though EGLD and ESDT transfers are now possible.
    ///
    /// ## Important!
    ///
    /// Does not cover multi-transfer scenarios properly, but left for backwards compatibility.
    ///
    /// Please use `.egld()` instead!
    ///
    /// For raw handling, `.egld_direct_non_strict()` is also acceptable.
    #[deprecated(
        since = "0.55.0",
        note = "Does not cover multi-transfer scenarios properly, but left for backwards compatibility. Please use .egld() instead!"
    )]
    pub fn egld_value(&self) -> ManagedRef<'static, A, BigUint<A>> {
        self.egld_direct_non_strict()
    }

    /// Returns the EGLD call value from the VM as ManagedDecimal
    pub fn egld_decimal(&self) -> ManagedDecimal<A, EgldDecimals> {
        ManagedDecimal::<A, EgldDecimals>::const_decimals_from_raw(self.egld_value().clone())
    }

    /// Returns all ESDT transfers that accompany this SC call.
    /// Will return 0 results if nothing was transferred, or just EGLD.
    ///
    /// Will crash for EGLD + ESDT multi transfers.
    pub fn all_esdt_transfers(&self) -> ManagedRef<'static, A, ManagedVec<A, EsdtTokenPayment<A>>> {
        let multi_esdt_handle: A::ManagedBufferHandle = self.all_esdt_transfers_unchecked();
        let checked = A::static_var_api_impl()
            .flag_is_set_or_update(StaticVarApiFlags::CALL_VALUE_ESDT_INITIALIZED);
        if !checked && egld_000000_transfer_exists::<A>(multi_esdt_handle.clone()) {
            A::error_api_impl().signal_error(err_msg::ESDT_UNEXPECTED_EGLD.as_bytes())
        }

        unsafe { ManagedRef::wrap_handle(multi_esdt_handle) }
    }

    /// Will return all transfers in the form of a list of EgldOrEsdtTokenPayment.
    ///
    /// Both EGLD and ESDT can be returned.
    ///
    /// In case of a single EGLD transfer, only one item will be returned,
    /// the EGLD payment represented as an ESDT transfer (EGLD-000000).
    pub fn all_transfers(
        &self,
    ) -> ManagedRef<'static, A, ManagedVec<A, EgldOrEsdtTokenPayment<A>>> {
        let all_transfers_handle: A::ManagedBufferHandle =
            use_raw_handle(const_handles::CALL_VALUE_ALL);
        if !A::static_var_api_impl()
            .flag_is_set_or_update(StaticVarApiFlags::CALL_VALUE_ALL_INITIALIZED)
        {
            A::call_value_api_impl().load_all_transfers(all_transfers_handle.clone());
        }
        unsafe { ManagedRef::wrap_handle(all_transfers_handle) }
    }

    /// Verify and casts the received multi ESDT transfer in to an array.
    ///
    /// Can be used to extract all payments in one line like this:
    ///
    /// `let [payment_a, payment_b, payment_c] = self.call_value().multi_esdt();`.
    ///
    /// Rejects EGLD transfers. Switch to `multi_egld_or_esdt` to accept mixed transfers.
    pub fn multi_esdt<const N: usize>(&self) -> [ManagedVecRef<'static, EsdtTokenPayment<A>>; N] {
        let esdt_transfers = self.all_esdt_transfers();
        let array = esdt_transfers.to_array_of_refs::<N>().unwrap_or_else(|| {
            A::error_api_impl().signal_error(err_msg::INCORRECT_NUM_ESDT_TRANSFERS.as_bytes())
        });
        unsafe { core::mem::transmute(array) }
    }

    /// Verify and casts the received multi ESDT transfer in to an array.
    ///
    /// Can be used to extract all payments in one line like this:
    ///
    /// `let [payment_a, payment_b, payment_c] = self.call_value().multi_egld_or_esdt();`.
    pub fn multi_egld_or_esdt<const N: usize>(
        &self,
    ) -> [ManagedVecRef<'static, EgldOrEsdtTokenPayment<A>>; N] {
        let esdt_transfers = self.all_transfers();
        let array = esdt_transfers.to_array_of_refs::<N>().unwrap_or_else(|| {
            A::error_api_impl().signal_error(err_msg::INCORRECT_NUM_TRANSFERS.as_bytes())
        });
        unsafe { core::mem::transmute(array) }
    }

    /// Expects precisely one ESDT token transfer, fungible or not.
    ///
    /// Will return the received ESDT payment.
    ///
    /// The amount cannot be 0, since that would not qualify as an ESDT transfer.
    pub fn single_esdt(&self) -> ManagedVecRef<'static, EsdtTokenPayment<A>> {
        let esdt_transfers = self.all_esdt_transfers();
        if esdt_transfers.len() != 1 {
            A::error_api_impl().signal_error(err_msg::INCORRECT_NUM_ESDT_TRANSFERS.as_bytes())
        }
        let value = esdt_transfers.get(0);
        unsafe { core::mem::transmute(value) }
    }

    /// Expects precisely one fungible ESDT token transfer.
    ///
    /// Returns the token ID and the amount for fungible ESDT transfers.
    ///
    /// The amount cannot be 0, since that would not qualify as an ESDT transfer.
    pub fn single_fungible_esdt(
        &self,
    ) -> (
        ManagedRef<'static, A, TokenIdentifier<A>>,
        ManagedRef<'static, A, BigUint<A>>,
    ) {
        let payment = self.single_esdt();
        if payment.token_nonce != 0 {
            A::error_api_impl().signal_error(err_msg::FUNGIBLE_TOKEN_EXPECTED_ERR_MSG.as_bytes());
        }

        unsafe {
            (
                ManagedRef::wrap_handle(payment.token_identifier.get_handle()),
                ManagedRef::wrap_handle(payment.amount.get_handle()),
            )
        }
    }

    /// Accepts and returns either an EGLD payment, or a single ESDT token.
    ///
    /// Will halt execution if more than one ESDT transfer was received.
    ///
    /// In case no transfer of value happen, it will return a payment of 0 EGLD.
    pub fn egld_or_single_esdt(&self) -> EgldOrEsdtTokenPayment<A> {
        let esdt_transfers_handle = self.all_esdt_transfers_unchecked();
        let esdt_transfers: ManagedRef<'static, A, ManagedVec<A, EgldOrEsdtTokenPayment<A>>> =
            unsafe { ManagedRef::wrap_handle(esdt_transfers_handle) };
        match esdt_transfers.len() {
            0 => EgldOrEsdtTokenPayment {
                token_identifier: EgldOrEsdtTokenIdentifier::egld(),
                token_nonce: 0,
                amount: self.egld_direct_non_strict().clone(),
            },
            1 => esdt_transfers.get(0).clone(),
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
            EgldOrMultiEsdtPayment::Egld(self.egld_direct_non_strict().clone())
        } else {
            EgldOrMultiEsdtPayment::MultiEsdt(esdt_transfers.clone())
        }
    }
}

fn egld_000000_transfer_exists<A>(transfers_vec_handle: A::ManagedBufferHandle) -> bool
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    A::managed_type_impl().mb_overwrite(
        use_raw_handle(const_handles::MBUF_EGLD_000000),
        EGLD_000000_TOKEN_IDENTIFIER.as_bytes(),
    );
    unsafe {
        let mut iter: ManagedVecPayloadIterator<
            A,
            <EsdtTokenPayment<A> as ManagedVecItem>::PAYLOAD,
        > = ManagedVecPayloadIterator::new(transfers_vec_handle);

        iter.any(|payload| {
            let token_identifier_handle = RawHandle::read_from_payload(payload.slice_unchecked(0));
            A::managed_type_impl().mb_eq(
                use_raw_handle(const_handles::MBUF_EGLD_000000),
                use_raw_handle(token_identifier_handle),
            )
        })
    }
}
