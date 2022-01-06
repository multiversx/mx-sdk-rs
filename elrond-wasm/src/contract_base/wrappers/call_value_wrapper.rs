use core::marker::PhantomData;

use crate::{
    api::{CallValueApi, CallValueApiImpl, ErrorApi, ManagedTypeApi},
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
        BigUint::from_raw_handle(A::call_value_api_impl().egld_value())
    }

    /// Returns all ESDT transfers that accompany this SC call.
    /// Will return 0 results if nothing was transfered, or just EGLD.
    /// Fully managed underlying types, very efficient.
    pub fn all_esdt_transfers(&self) -> ManagedVec<A, EsdtTokenPayment<A>> {
        A::call_value_api_impl().get_all_esdt_transfers()
    }

    /// Retrieves the ESDT call value from the VM.
    /// Will return 0 in case of an EGLD transfer (cannot have both EGLD and ESDT transfer simultaneously).
    /// Warning, not tested with multi transfer, use `all_esdt_transfers` instead!
    pub fn esdt_value(&self) -> BigUint<A> {
        BigUint::from_raw_handle(A::call_value_api_impl().esdt_value())
    }

    /// Returns the call value token identifier of the current call.
    /// The identifier is wrapped in a TokenIdentifier object, to hide underlying logic.
    ///
    /// A note on implementation: even though the underlying api returns an empty name for EGLD,
    /// but the EGLD TokenIdentifier is serialized as `EGLD`.
    /// Warning, not tested with multi transfer, use `all_esdt_transfers` instead!
    pub fn token(&self) -> TokenIdentifier<A> {
        TokenIdentifier::from_raw_handle(A::call_value_api_impl().token())
    }

    /// Returns the nonce of the received ESDT token.
    /// Will return 0 in case of EGLD or fungible ESDT transfer.
    /// Warning, not tested with multi transfer, use `all_esdt_transfers` instead!
    pub fn esdt_token_nonce(&self) -> u64 {
        A::call_value_api_impl().esdt_token_nonce()
    }

    /// Returns the ESDT token type.
    /// Will return "Fungible" for EGLD.
    /// Warning, not tested with multi transfer, use `all_esdt_transfers` instead!
    pub fn esdt_token_type(&self) -> EsdtTokenType {
        A::call_value_api_impl().esdt_token_type()
    }

    pub fn require_egld(&self) -> BigUint<A> {
        BigUint::from_raw_handle(A::call_value_api_impl().require_egld())
    }

    pub fn require_esdt(&self, token: &[u8]) -> BigUint<A> {
        BigUint::from_raw_handle(A::call_value_api_impl().require_esdt(token))
    }

    /// Returns both the call value (either EGLD or ESDT) and the token identifier.
    /// Especially used in the `#[payable("*")] auto-generated snippets.
    /// The method might seem redundant, but there is such a hook in Arwen
    /// that might be used in this scenario in the future.
    /// TODO: replace with multi transfer handling everywhere
    pub fn payment_token_pair(&self) -> (BigUint<A>, TokenIdentifier<A>) {
        let (amount_handle, token_handle) = A::call_value_api_impl().payment_token_pair();
        (
            BigUint::from_raw_handle(amount_handle),
            TokenIdentifier::from_raw_handle(token_handle),
        )
    }
}
