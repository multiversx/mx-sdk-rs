use crate::types::{BigUint, EsdtTokenPayment, EsdtTokenType, ManagedVec, TokenIdentifier} ;

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

    /// Will return the EGLD call value,
    /// but also fail with an error if ESDT is sent.
    /// Especially used in the auto-generated call value processing.
    pub fn require_egld(&self) -> BigUint<A> {
        if !self.token().is_egld() {
            self.signal_error(err_msg::NON_PAYABLE_FUNC_ESDT);
        }
        self.egld_value()
    }

    /// Will return the ESDT call value,
    /// but also fail with an error if EGLD or the wrong ESDT token is sent.
    /// Especially used in the auto-generated call value processing.
    pub fn require_esdt(&self, token: &[u8]) -> BigUint<A> {
        if self.token().as_managed_buffer() != token {
            self.signal_error(err_msg::BAD_TOKEN_PROVIDED) ;
        }
        self.esdt_value()
    }

    /// Returns both the call value (either EGLD or ESDT) and the token identifier.
    /// Especially used in the `#[payable("*")] auto-generated snippets.
    /// The method might seem redundant, but there is such a hook in Arwen
    /// that might be used in this scenario in the future.
    pub fn payment_token_pair(
        &self,
    ) -> (
        BigUint<A>,
        TokenIdentifier<A>,
    ) {
        let token = self.token() ;
        if token.is_egld() {
            (self.egld_value(), token)
        } else {
            (self.esdt_value(), token)
        }
    }

    pub fn esdt_num_transfers(&self) -> usize {
        self.api.esdt_num_transfers()
    }

    pub fn esdt_value_by_index(&self, index: usize) -> BigUint<A> {
        self.api.esdt_value_by_index()
    }

    pub fn token_by_index(&self, index: usize) -> TokenIdentifier<A> {
        self.api.token_by_index()
    }

    pub fn esdt_token_nonce_by_index(&self, index: usize) -> u64 {
        self.api.esdt_token_nonce_by_index()
    }

    pub fn esdt_token_type_by_index(&self, index: usize) -> EsdtTokenType {
        self.api.esdt_token_type_by_index()
    }

    pub fn get_all_esdt_transfers(
        &self,
    ) -> ManagedVec<A, EsdtTokenPayment<A>>  {
        self.api.get_all_esdt_transfers()
    }
}
