use crate::{
    api::{CallValueApi, ErrorApi, ManagedTypeApi},
    types::{BigUint, EsdtTokenPayment, EsdtTokenType, ManagedVec, TokenIdentifier},
};

pub struct CallValueWrapper<A>
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    pub(crate) api: A,
}

impl<A> CallValueWrapper<A>
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    pub(crate) fn new(api: A) -> Self {
        CallValueWrapper { api }
    }

    /// Retrieves the EGLD call value from the VM.
    /// Will return 0 in case of an ESDT transfer (cannot have both EGLD and ESDT transfer simultaneously).
    pub fn egld_value(&self) -> BigUint<A> {
        self.api.egld_value()
    }

    /// Returns all ESDT transfers that accompany this SC call.
    /// Will return 0 results if nothing was transfered, or just EGLD.
    /// Fully managed underlying types, very efficient.
    pub fn all_esdt_transfers(&self) -> ManagedVec<A, EsdtTokenPayment<A>> {
        self.api.get_all_esdt_transfers()
    }

    /// Retrieves the ESDT call value from the VM.
    /// Will return 0 in case of an EGLD transfer (cannot have both EGLD and ESDT transfer simultaneously).
    /// Warning, not tested with multi transfer, use `all_esdt_transfers` instead!
    pub fn esdt_value(&self) -> BigUint<A> {
        self.api.esdt_value()
    }

    /// Returns the call value token identifier of the current call.
    /// The identifier is wrapped in a TokenIdentifier object, to hide underlying logic.
    ///
    /// A note on implementation: even though the underlying api returns an empty name for EGLD,
    /// but the EGLD TokenIdentifier is serialized as `EGLD`.
    /// Warning, not tested with multi transfer, use `all_esdt_transfers` instead!
    pub fn token(&self) -> TokenIdentifier<A> {
        self.api.token()
    }

    /// Returns the nonce of the received ESDT token.
    /// Will return 0 in case of EGLD or fungible ESDT transfer.
    /// Warning, not tested with multi transfer, use `all_esdt_transfers` instead!
    pub fn esdt_token_nonce(&self) -> u64 {
        self.api.esdt_token_nonce()
    }

    /// Returns the ESDT token type.
    /// Will return "Fungible" for EGLD.
    /// Warning, not tested with multi transfer, use `all_esdt_transfers` instead!
    pub fn esdt_token_type(&self) -> EsdtTokenType {
        self.api.esdt_token_type()
    }

    /// Returns both the call value (either EGLD or ESDT) and the token identifier.
    /// Especially used in the `#[payable("*")] auto-generated snippets.
    /// The method might seem redundant, but there is such a hook in Arwen
    /// that might be used in this scenario in the future.
    /// TODO: replace with multi transfer handling everywhere
    pub fn payment_token_pair(&self) -> (BigUint<A>, TokenIdentifier<A>) {
        self.api.payment_token_pair()
    }
}
