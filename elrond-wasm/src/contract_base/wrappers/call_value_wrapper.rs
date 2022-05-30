use core::marker::PhantomData;

use crate::{
    api::{
        const_handles, CallValueApi, CallValueApiImpl, ErrorApi, ErrorApiImpl, ManagedTypeApi,
        StaticVarApiImpl,
    },
    err_msg,
    types::{
        BigUint, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment, EsdtTokenPayment,
        EsdtTokenType, ManagedType, ManagedVec, TokenIdentifier,
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
    pub fn egld_value(&self) -> BigUint<A> {
        let mut call_value_handle = A::static_var_api_impl().get_call_value_egld_handle();
        if call_value_handle == const_handles::UNINITIALIZED_HANDLE {
            call_value_handle = const_handles::CALL_VALUE_EGLD;
            A::static_var_api_impl().set_call_value_egld_handle(call_value_handle);
            A::call_value_api_impl().load_egld_value(call_value_handle);
        }
        BigUint::from_raw_handle(call_value_handle) // unsafe, TODO: replace with ManagedRef<...>
    }

    /// Returns all ESDT transfers that accompany this SC call.
    /// Will return 0 results if nothing was transfered, or just EGLD.
    /// Fully managed underlying types, very efficient.
    pub fn all_esdt_transfers(&self) -> ManagedVec<A, EsdtTokenPayment<A>> {
        let mut call_value_handle = A::static_var_api_impl().get_call_value_multi_esdt_handle();
        if call_value_handle == const_handles::UNINITIALIZED_HANDLE {
            call_value_handle = const_handles::CALL_VALUE_MULTI_ESDT;
            A::static_var_api_impl().set_call_value_multi_esdt_handle(call_value_handle);
            A::call_value_api_impl().load_all_esdt_transfers(call_value_handle);
        }
        ManagedVec::from_raw_handle(call_value_handle) // unsafe, TODO: replace with ManagedRef<...>
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

    pub fn single_esdt(&self) -> EsdtTokenPayment<A> {
        let [payments] = self.multi_esdt();
        payments
    }

    /// Retrieves the ESDT call value from the VM.
    /// Will return 0 in case of an EGLD transfer (cannot have both EGLD and ESDT transfer simultaneously).
    pub fn esdt_value(&self) -> BigUint<A> {
        A::call_value_api_impl().load_single_esdt_value(const_handles::CALL_VALUE_SINGLE_ESDT);
        BigUint::from_raw_handle(const_handles::CALL_VALUE_SINGLE_ESDT)
    }

    pub fn egld_or_single_esdt(&self) -> EgldOrEsdtTokenPayment<A> {
        let esdt_transfers = self.all_esdt_transfers();
        match esdt_transfers.len() {
            0 => EgldOrEsdtTokenPayment {
                token_identifier: EgldOrEsdtTokenIdentifier::egld(),
                token_nonce: 0,
                amount: self.egld_value(),
            },
            1 => esdt_transfers.get(0).into(),
            _ => A::error_api_impl().signal_error(err_msg::INCORRECT_NUM_ESDT_TRANSFERS.as_bytes()),
        }
    }

    /// Returns the call value token identifier of the current call.
    /// The identifier is wrapped in an EgldOrEsdtTokenIdentifier object, to hide underlying logic.
    ///
    /// A note on implementation: even though the underlying api returns an empty name for EGLD,
    /// but the EGLD token ID is serialized as `EGLD`.
    /// Calling this when receiving a multi-token transfer will signal an error.
    pub fn token(&self) -> EgldOrEsdtTokenIdentifier<A> {
        let call_value_api = A::call_value_api_impl();
        let error_api = A::error_api_impl();

        match call_value_api.esdt_num_transfers() {
            0 => EgldOrEsdtTokenIdentifier::egld(),
            1 => EgldOrEsdtTokenIdentifier::esdt(TokenIdentifier::from_raw_handle(
                call_value_api.token(),
            )),
            _ => error_api.signal_error(err_msg::TOO_MANY_ESDT_TRANSFERS.as_bytes()),
        }
    }

    /// Returns the nonce of the received ESDT token.
    /// Will return 0 in case of EGLD or fungible ESDT transfer.
    pub fn esdt_token_nonce(&self) -> u64 {
        let call_value_api = A::call_value_api_impl();
        if call_value_api.esdt_num_transfers() > 0 {
            call_value_api.esdt_token_nonce()
        } else {
            0
        }
    }

    /// Returns the ESDT token type.
    /// Will return "Fungible" for EGLD.
    pub fn esdt_token_type(&self) -> EsdtTokenType {
        let call_value_api = A::call_value_api_impl();
        if call_value_api.esdt_num_transfers() > 0 {
            A::call_value_api_impl().esdt_token_type()
        } else {
            EsdtTokenType::Fungible
        }
    }

    /// Returns the token ID and the amount for fungible ESDT transfers
    /// Will signal an error for EGLD or non-fungible token payments
    pub fn single_fungible_esdt_payment(&self) -> (TokenIdentifier<A>, BigUint<A>) {
        let call_value_api = A::call_value_api_impl();
        if call_value_api.esdt_num_transfers() == 0 {
            A::error_api_impl().signal_error(err_msg::NO_PAYMENT_ERR_MSG);
        }
        if self.esdt_token_nonce() != 0 {
            A::error_api_impl().signal_error(err_msg::FUNGIBLE_TOKEN_EXPECTED_ERR_MSG);
        }

        (self.token().unwrap_esdt(), self.esdt_value())
    }

    pub fn payment(&self) -> EgldOrEsdtTokenPayment<A> {
        let api = A::call_value_api_impl();
        if api.esdt_num_transfers() == 0 {
            EgldOrEsdtTokenPayment::new(EgldOrEsdtTokenIdentifier::egld(), 0, self.egld_value())
        } else {
            EgldOrEsdtTokenPayment::new(self.token(), self.esdt_token_nonce(), self.esdt_value())
        }
    }

    pub fn payment_as_tuple(&self) -> (EgldOrEsdtTokenIdentifier<A>, u64, BigUint<A>) {
        let payment = self.payment();

        (
            payment.token_identifier,
            payment.token_nonce,
            payment.amount,
        )
    }
}
