use crate::{action::Action, user_role::UserRole};

elrond_wasm::imports!();

/// Contains all events that can be emitted by the contract.
#[elrond_wasm::module]
pub trait MultisigEventsModule {
    #[event("sign")]
    fn sign_event(&self, #[indexed] signer: &ManagedAddress, #[indexed] action_id: usize);

    #[event("unsign")]
    fn unsign_event(&self, #[indexed] signer: &ManagedAddress, #[indexed] action_id: usize);

    #[event("discardAction")]
    fn discard_action_event(&self, #[indexed] signer: &ManagedAddress, #[indexed] action_id: usize);

    #[event("performChangeUser")]
    fn perform_change_user_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] action_id: usize,
        #[indexed] changed_user: &ManagedAddress,
        #[indexed] old_role: UserRole,
        #[indexed] new_role: UserRole,
    );
}
