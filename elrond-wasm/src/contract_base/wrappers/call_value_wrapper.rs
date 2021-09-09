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

    pub fn check_not_payable(&self) {
        self.api.check_not_payable()
    }

    /// Retrieves the EGLD call value from the VM.
    /// Will return 0 in case of an ESDT transfer (cannot have both EGLD and ESDT transfer simultaneously).
    pub fn egld_value(&self) -> BigUint<A> {
        self.api.egld_value()
    }

    /// Retrieves the ESDT call value from the VM.
    /// Will return 0 in case of an EGLD transfer (cannot have both EGLD and ESDT transfer simultaneously).
    pub fn esdt_value(&self) -> BigUint<A> {
        self.api.esdt_value()
    }

    /// Returns the call value token identifier of the current call.
    /// The identifier is wrapped in a TokenIdentifier object, to hide underlying logic.
    ///
    /// A note on implementation: even though the underlying api returns an empty name for EGLD,
    /// but the EGLD TokenIdentifier is serialized as `EGLD`.
    pub fn token(&self) -> TokenIdentifier<A> {
        self.api.token()
    }

    /// Returns the nonce of the received ESDT token.
    /// Will return 0 in case of EGLD or fungible ESDT transfer.
    pub fn esdt_token_nonce(&self) -> u64 {
        self.api.esdt_token_nonce()
    }

    /// Returns the ESDT token type.
    /// Will return "Fungible" for EGLD.
    pub fn esdt_token_type(&self) -> EsdtTokenType {
        self.api.esdt_token_type()
    }

    /// Returns both the call value (either EGLD or ESDT) and the token identifier.
    /// Especially used in the `#[payable("*")] auto-generated snippets.
    /// The method might seem redundant, but there is such a hook in Arwen
    /// that might be used in this scenario in the future.
    pub fn payment_token_pair(&self) -> (BigUint<A>, TokenIdentifier<A>) {
        let token = self.token();
        if token.is_egld() {
            (self.egld_value(), token)
        } else {
            (self.esdt_value(), token)
        }
    }

    pub fn get_all_esdt_transfers(&self) -> ManagedVec<A, EsdtTokenPayment<A>> {
        self.api.get_all_esdt_transfers()
    }
}
