use super::{ErrorApiImpl, Handle, ManagedTypeApi, ManagedTypeApiImpl};
use crate::{
    err_msg,
    types::{BigUint, EsdtTokenPayment, EsdtTokenType, ManagedType, ManagedVec, TokenIdentifier},
};

const FIRST_TOKEN_INDEX: usize = 0;

pub trait CallValueApi {
    type CallValueApiImpl: CallValueApiImpl;

    fn call_value_api_impl() -> Self::CallValueApiImpl;
}

pub trait CallValueApiImpl: ErrorApiImpl + ManagedTypeApiImpl {
    fn check_not_payable(&self);

    fn require_max_one_esdt_transfer(&self) {
        if self.esdt_num_transfers() > 1 {
            self.signal_error(err_msg::TOO_MANY_ESDT_TRANSFERS);
        }
    }

    /// Retrieves the EGLD call value from the VM.
    /// Will return 0 in case of an ESDT transfer (cannot have both EGLD and ESDT transfer simultaneously).
    fn egld_value(&self) -> Handle;

    /// Retrieves the ESDT call value from the VM.
    /// Will return 0 in case of an EGLD transfer (cannot have both EGLD and ESDT transfer simultaneously).
    fn esdt_value(&self) -> Handle {
        self.require_max_one_esdt_transfer();
        self.esdt_value_by_index(FIRST_TOKEN_INDEX)
    }

    /// Returns the call value token identifier of the current call.
    /// The identifier is wrapped in a TokenIdentifier object, to hide underlying logic.
    ///
    /// A note on implementation: even though the underlying api returns an empty name for EGLD,
    /// but the EGLD TokenIdentifier is serialized as `EGLD`.
    fn token(&self) -> Handle {
        self.require_max_one_esdt_transfer();
        self.token_by_index(FIRST_TOKEN_INDEX)
    }

    /// Returns the nonce of the received ESDT token.
    /// Will return 0 in case of EGLD or fungible ESDT transfer.
    fn esdt_token_nonce(&self) -> u64 {
        self.require_max_one_esdt_transfer();
        self.esdt_token_nonce_by_index(FIRST_TOKEN_INDEX)
    }

    /// Returns the ESDT token type.
    /// Will return "Fungible" for EGLD.
    fn esdt_token_type(&self) -> EsdtTokenType {
        self.require_max_one_esdt_transfer();
        self.esdt_token_type_by_index(FIRST_TOKEN_INDEX)
    }

    /// Will return the EGLD call value,
    /// but also fail with an error if ESDT is sent.
    /// Especially used in the auto-generated call value processing.
    fn require_egld(&self) -> Handle {
        if self.esdt_num_transfers() > 0 {
            self.signal_error(err_msg::NON_PAYABLE_FUNC_ESDT);
        }
        self.egld_value()
    }

    /// Will return the ESDT call value,
    /// but also fail with an error if EGLD or the wrong ESDT token is sent.
    /// Especially used in the auto-generated call value processing.
    fn require_esdt(&self, token: &[u8]) -> Handle {
        let want = self.mb_new_from_bytes(token);
        if !self.mb_eq(self.token(), want) {
            self.signal_error(err_msg::BAD_TOKEN_PROVIDED);
        }
        self.esdt_value()
    }

    /// Returns both the call value (either EGLD or ESDT) and the token identifier.
    /// Especially used in the `#[payable("*")] auto-generated snippets.
    /// The method might seem redundant, but there is such a hook
    /// that might be used in this scenario in the future.
    fn payment_token_pair(&self) -> (Handle, Handle) {
        let token = self.token();
        if self.esdt_num_transfers() == 0 {
            (self.egld_value(), token)
        } else {
            (self.esdt_value(), token)
        }
    }

    fn esdt_num_transfers(&self) -> usize;

    fn esdt_value_by_index(&self, index: usize) -> Handle;

    fn token_by_index(&self, index: usize) -> Handle;

    fn esdt_token_nonce_by_index(&self, index: usize) -> u64;

    fn esdt_token_type_by_index(&self, index: usize) -> EsdtTokenType;

    fn get_all_esdt_transfers<M: ManagedTypeApi>(&self) -> ManagedVec<M, EsdtTokenPayment<M>> {
        let num_transfers = self.esdt_num_transfers();
        let mut transfers = ManagedVec::new();

        for i in 0..num_transfers {
            let token_type = self.esdt_token_type_by_index(i);
            let token_identifier = TokenIdentifier::from_raw_handle(self.token_by_index(i));
            let token_nonce = self.esdt_token_nonce_by_index(i);
            let amount = BigUint::from_raw_handle(self.esdt_value_by_index(i));

            transfers.push(EsdtTokenPayment::<M> {
                token_type,
                token_identifier,
                token_nonce,
                amount,
            });
        }

        transfers
    }
}
