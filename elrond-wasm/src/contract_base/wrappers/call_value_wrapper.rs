use core::marker::PhantomData;

use crate::{
    api::{
        const_handles, CallValueApi, CallValueApiImpl, ErrorApi, ErrorApiImpl, ManagedTypeApi,
        StaticVarApiImpl,
    },
    err_msg,
    types::{BigUint, EsdtTokenPayment, EsdtTokenType, ManagedType, ManagedVec, TokenIdentifier},
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

    /// Retrieves the ESDT call value from the VM.
    /// Will return 0 in case of an EGLD transfer (cannot have both EGLD and ESDT transfer simultaneously).
    pub fn esdt_value(&self) -> BigUint<A> {
        A::call_value_api_impl().load_single_esdt_value(const_handles::CALL_VALUE_SINGLE_ESDT);
        BigUint::from_raw_handle(const_handles::CALL_VALUE_SINGLE_ESDT)
    }

    /// Returns the call value token identifier of the current call.
    /// The identifier is wrapped in a TokenIdentifier object, to hide underlying logic.
    ///
    /// A note on implementation: even though the underlying api returns an empty name for EGLD,
    /// but the EGLD TokenIdentifier is serialized as `EGLD`.
    /// Calling this when receiving a multi-token transfer will signal an error.
    pub fn token(&self) -> TokenIdentifier<A> {
        let call_value_api = A::call_value_api_impl();
        let error_api = A::error_api_impl();

        match call_value_api.esdt_num_transfers() {
            0 => TokenIdentifier::egld(),
            1 => TokenIdentifier::from_raw_handle(call_value_api.token()),
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

    /// Returns both the call value (either EGLD or ESDT) and the token identifier.
    /// Especially used in the `#[payable("*")] auto-generated snippets.
    /// TODO: replace with multi transfer handling everywhere
    pub fn payment_token_pair(&self) -> (BigUint<A>, TokenIdentifier<A>) {
        let call_value_api = A::call_value_api_impl();
        if call_value_api.esdt_num_transfers() == 0 {
            (self.egld_value(), TokenIdentifier::egld())
        } else {
            (self.esdt_value(), self.token())
        }
    }

    pub fn payment(&self) -> EsdtTokenPayment<A> {
        let api = A::call_value_api_impl();
        if api.esdt_num_transfers() == 0 {
            EsdtTokenPayment::new(TokenIdentifier::egld(), 0, self.egld_value())
        } else {
            EsdtTokenPayment::new(self.token(), self.esdt_token_nonce(), self.esdt_value())
        }
    }

    pub fn payment_as_tuple(&self) -> (TokenIdentifier<A>, u64, BigUint<A>) {
        let (amount, token) = self.payment_token_pair();
        let nonce = self.esdt_token_nonce();

        (token, nonce, amount)
    }
}
