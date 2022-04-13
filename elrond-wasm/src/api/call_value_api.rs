use super::{ErrorApiImpl, Handle, ManagedTypeApi, ManagedTypeApiImpl};
use crate::types::{
    BigUint, EsdtTokenPayment, EsdtTokenType, ManagedType, ManagedVec, TokenIdentifier,
};

pub trait CallValueApi {
    type CallValueApiImpl: CallValueApiImpl;

    fn call_value_api_impl() -> Self::CallValueApiImpl;
}

pub trait CallValueApiImpl: ErrorApiImpl + ManagedTypeApiImpl {
    fn check_not_payable(&self);

    /// Retrieves the EGLD call value from the VM.
    /// Will return 0 in case of an ESDT transfer (cannot have both EGLD and ESDT transfer simultaneously).
    fn load_egld_value(&self, dest: Handle);

    /// Retrieves the ESDT call value from the VM.
    /// Will return 0 in case of an EGLD transfer (cannot have both EGLD and ESDT transfer simultaneously).
    fn load_single_esdt_value(&self, dest: Handle);

    /// Returns the call value token identifier of the current call.
    /// The identifier is wrapped in a TokenIdentifier object, to hide underlying logic.
    ///
    /// A note on implementation: even though the underlying api returns an empty name for EGLD,
    /// but the EGLD TokenIdentifier is serialized as `EGLD`.
    fn token(&self) -> Handle;

    /// Returns the nonce of the received ESDT token.
    /// Will return 0 in case of EGLD or fungible ESDT transfer.
    fn esdt_token_nonce(&self) -> u64;

    /// Returns the ESDT token type.
    /// Will return "Fungible" for EGLD.
    fn esdt_token_type(&self) -> EsdtTokenType;

    /// Will return the ESDT call value,
    /// but also fail with an error if EGLD or the wrong ESDT token is sent.
    /// Especially used in the auto-generated call value processing.
    ///
    /// TODO: rename to `require_single_esdt`.
    // fn require_esdt(&self, token: &[u8]) -> Handle {
    //     let want = self.mb_new_from_bytes(token);
    //     if self.esdt_num_transfers() != 1 {
    //         self.signal_error(err_msg::SINGLE_ESDT_EXPECTED.as_bytes());
    //     }
    //     if !self.mb_eq(self.token(), want) {
    //         self.signal_error(err_msg::BAD_TOKEN_PROVIDED.as_bytes());
    //     }
    //     self.esdt_value(const_handles::CALL_VALUE_SINGLE_ESDT);
    //     const_handles::CALL_VALUE_SINGLE_ESDT
    // }

    /// Returns both the call value (either EGLD or ESDT) and the token identifier.
    /// Especially used in the `#[payable("*")] auto-generated snippets.
    /// The method might seem redundant, but there is such a hook in Arwen
    /// that might be used in this scenario in the future.
    // fn payment_token_pair(&self) -> (Handle, Handle) {
    //     if self.esdt_num_transfers() == 0 {
    //         (self.egld_value(), self.mb_new_empty())
    //     } else {
    //         (self.esdt_value(), self.token())
    //     }
    // }

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
