use core::marker::PhantomData;

use crate::{
    api::{
        const_handles, use_raw_handle, CallValueApi, CallValueApiImpl, ErrorApi, ErrorApiImpl,
        HandleConstraints, ManagedBufferApiImpl, ManagedTypeApi, RawHandle, StaticVarApiImpl,
    },
    err_msg,
    types::{
        BigUint, ConstDecimals, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment,
        EgldOrMultiEsdtPayment, EsdtTokenPayment, ManagedDecimal, ManagedRef, ManagedType,
        ManagedVec, ManagedVecItem, ManagedVecItemPayload, ManagedVecPayloadIterator,
        ManagedVecRef, TokenIdentifier, EGLD_000000_TOKEN_IDENTIFIER,
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
        let (egld_amount_handle, _) = load_all_call_data::<A>();
        unsafe { ManagedRef::wrap_handle(egld_amount_handle) }
    }

    /// Returns the EGLD call value from the VM as ManagedDecimal
    pub fn egld_decimal(&self) -> ManagedDecimal<A, ConstDecimals<18>> {
        ManagedDecimal::<A, ConstDecimals<18>>::const_decimals_from_raw(
            self.egld_value().clone_value(),
        )
    }

    /// Returns all ESDT transfers that accompany this SC call.
    /// Will return 0 results if nothing was transfered, or just EGLD.
    /// Fully managed underlying types, very efficient.
    pub fn all_esdt_transfers(&self) -> ManagedRef<'static, A, ManagedVec<A, EsdtTokenPayment<A>>> {
        let call_value_handle = load_all_transfers::<A>();

        let egld_payment = find_egld_000000_transfer::<A>(call_value_handle.clone());
        if egld_payment.is_some() {
            A::error_api_impl().signal_error(err_msg::ESDT_UNEXPECTED_EGLD.as_bytes())
        }
        unsafe { ManagedRef::wrap_handle(call_value_handle) }
    }

    pub fn all_transfers(
        &self,
    ) -> ManagedRef<'static, A, ManagedVec<A, EgldOrEsdtTokenPayment<A>>> {
        let (_, all_transfers_handle) = load_all_call_data::<A>();
        unsafe { ManagedRef::wrap_handle(all_transfers_handle) }
    }

    /// Verify and casts the received multi ESDT transfer in to an array.
    ///
    /// Can be used to extract all payments in one line like this:
    ///
    /// `let [payment_a, payment_b, payment_c] = self.call_value().multi_esdt();`.
    pub fn multi_esdt<const N: usize>(&self) -> [ManagedVecRef<'static, EsdtTokenPayment<A>>; N] {
        let esdt_transfers = self.all_esdt_transfers();
        let array = esdt_transfers.to_array_of_refs::<N>().unwrap_or_else(|| {
            A::error_api_impl().signal_error(err_msg::INCORRECT_NUM_ESDT_TRANSFERS.as_bytes())
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
        let esdt_transfers = self.all_esdt_transfers();
        match esdt_transfers.len() {
            0 => EgldOrEsdtTokenPayment {
                token_identifier: EgldOrEsdtTokenIdentifier::egld(),
                token_nonce: 0,
                amount: self.egld_value().clone_value(),
            },
            1 => esdt_transfers.get(0).clone().into(),
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

fn load_all_transfers<A>() -> A::ManagedBufferHandle
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    let mut all_transfers_handle: A::ManagedBufferHandle =
        use_raw_handle(A::static_var_api_impl().get_call_value_multi_esdt_handle());
    if all_transfers_handle == const_handles::UNINITIALIZED_HANDLE {
        all_transfers_handle = use_raw_handle(const_handles::CALL_VALUE_MULTI_ESDT);
        A::static_var_api_impl()
            .set_call_value_multi_esdt_handle(all_transfers_handle.get_raw_handle());
        A::call_value_api_impl().load_all_esdt_transfers(all_transfers_handle.clone());
    }
    all_transfers_handle
}

fn find_egld_000000_transfer<A>(transfers_vec_handle: A::ManagedBufferHandle) -> Option<RawHandle>
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

        if iter.remaining_count() <= 1 {
            // EGLD is not allowed in single transfers
            return None;
        }

        let egld_payload = iter.find(|payload| {
            let token_identifier_handle = RawHandle::read_from_payload(payload.slice_unchecked(0));
            A::managed_type_impl().mb_eq(
                use_raw_handle(const_handles::MBUF_EGLD_000000),
                use_raw_handle(token_identifier_handle),
            )
        });

        egld_payload.map(|payload| RawHandle::read_from_payload(payload.slice_unchecked(12)))
    }
}

fn load_all_call_data<A>() -> (A::BigIntHandle, A::ManagedBufferHandle)
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    let all_transfers_handle = load_all_transfers::<A>();
    let mut egld_amount_handle: A::BigIntHandle =
        use_raw_handle(A::static_var_api_impl().get_call_value_egld_handle());
    if egld_amount_handle == const_handles::UNINITIALIZED_HANDLE {
        let egld_payment = find_egld_000000_transfer::<A>(all_transfers_handle.clone());
        if let Some(egld_amount_raw_handle) = egld_payment {
            egld_amount_handle = use_raw_handle(egld_amount_raw_handle);
        } else {
            egld_amount_handle = use_raw_handle(const_handles::CALL_VALUE_EGLD);
            A::call_value_api_impl().load_egld_value(egld_amount_handle.clone());
        }
        A::static_var_api_impl().set_call_value_egld_handle(egld_amount_handle.get_raw_handle());
    }

    (egld_amount_handle, all_transfers_handle)
}
