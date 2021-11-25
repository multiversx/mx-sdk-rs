use super::{ErrorApi, ManagedTypeApi};
use crate::{
    err_msg,
    types::{BigUint, EsdtTokenPayment, EsdtTokenType, ManagedVec, TokenIdentifier},
};

pub trait CallValueApi: ManagedTypeApi + ErrorApi + Sized {
    fn check_not_payable(&self);

    /// Retrieves the EGLD call value from the VM.
    /// Will return 0 in case of an ESDT transfer (cannot have both EGLD and ESDT transfer simultaneously).
    fn egld_value(&self) -> BigUint<Self>;

    /// Retrieves the ESDT call value from the VM.
    /// Will return 0 in case of an EGLD transfer (cannot have both EGLD and ESDT transfer simultaneously).
    fn esdt_value(&self) -> BigUint<Self>;

    /// Returns the call value token identifier of the current call.
    /// The identifier is wrapped in a TokenIdentifier object, to hide underlying logic.
    ///
    /// A note on implementation: even though the underlying api returns an empty name for EGLD,
    /// but the EGLD TokenIdentifier is serialized as `EGLD`.
    fn token(&self) -> TokenIdentifier<Self>;

    /// Returns the nonce of the received ESDT token.
    /// Will return 0 in case of EGLD or fungible ESDT transfer.
    fn esdt_token_nonce(&self) -> u64;

    /// Returns the ESDT token type.
    /// Will return "Fungible" for EGLD.
    fn esdt_token_type(&self) -> EsdtTokenType;

    /// Will return the EGLD call value,
    /// but also fail with an error if ESDT is sent.
    /// Especially used in the auto-generated call value processing.
    fn require_egld(&self) -> BigUint<Self> {
        if !self.token().is_egld() {
            self.signal_error(err_msg::NON_PAYABLE_FUNC_ESDT);
        }
        self.egld_value()
    }

    /// Will return the ESDT call value,
    /// but also fail with an error if EGLD or the wrong ESDT token is sent.
    /// Especially used in the auto-generated call value processing.
    fn require_esdt(&self, token: &[u8]) -> BigUint<Self> {
        if self.token().as_managed_buffer() != token {
            self.signal_error(err_msg::BAD_TOKEN_PROVIDED);
        }
        self.esdt_value()
    }

    /// Returns both the call value (either EGLD or ESDT) and the token identifier.
    /// Especially used in the `#[payable("*")] auto-generated snippets.
    /// The method might seem redundant, but there is such a hook in Arwen
    /// that might be used in this scenario in the future.
    fn payment_token_pair(&self) -> (BigUint<Self>, TokenIdentifier<Self>) {
        let token = self.token();
        if token.is_egld() {
            (self.egld_value(), token)
        } else {
            (self.esdt_value(), token)
        }
    }

    fn esdt_num_transfers(&self) -> usize;

    fn esdt_value_by_index(&self, index: usize) -> BigUint<Self>;

    fn token_by_index(&self, index: usize) -> TokenIdentifier<Self>;

    fn esdt_token_nonce_by_index(&self, index: usize) -> u64;

    fn esdt_token_type_by_index(&self, index: usize) -> EsdtTokenType;

    fn get_all_esdt_transfers(&self) -> ManagedVec<Self, EsdtTokenPayment<Self>> {
        let num_transfers = self.esdt_num_transfers();
        let mut transfers = ManagedVec::new();

        for i in 0..num_transfers {
            let token_type = self.esdt_token_type_by_index(i);
            let token_identifier = self.token_by_index(i);
            let token_nonce = self.esdt_token_nonce_by_index(i);
            let amount = self.esdt_value_by_index(i);

            transfers.push(EsdtTokenPayment::<Self> {
                token_type,
                token_identifier,
                token_nonce,
                amount,
            });
        }

        transfers
    }
}
