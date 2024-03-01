use crate::{action::ActionFullInfo, user_role::UserRole};

multiversx_sc::imports!();

/// Contains all events that can be emitted by the contract.
#[multiversx_sc::module]
pub trait MultisigEventsModule {
    #[event("startPerformAction")]
    fn start_perform_action_event(&self, data: &ActionFullInfo<Self::Api>);

    #[event("performChangeUser")]
    fn perform_change_user_event(
        &self,
        #[indexed] action_id: usize,
        #[indexed] changed_user: &ManagedAddress,
        #[indexed] old_role: UserRole,
        #[indexed] new_role: UserRole,
    );

    #[event("performChangeQuorum")]
    fn perform_change_quorum_event(
        &self,
        #[indexed] action_id: usize,
        #[indexed] new_quorum: usize,
    );

    #[event("performAsyncCall")]
    fn perform_async_call_event(
        &self,
        #[indexed] action_id: usize,
        #[indexed] to: &ManagedAddress,
        #[indexed] egld_value: &BigUint,
        #[indexed] gas: u64,
        #[indexed] endpoint: &ManagedBuffer,
        #[indexed] arguments: &MultiValueManagedVec<ManagedBuffer>,
    );

    #[event("performTransferExecute")]
    fn perform_transfer_execute_event(
        &self,
        #[indexed] action_id: usize,
        #[indexed] to: &ManagedAddress,
        #[indexed] egld_value: &BigUint,
        #[indexed] gas: u64,
        #[indexed] endpoint: &ManagedBuffer,
        #[indexed] arguments: &MultiValueManagedVec<ManagedBuffer>,
    );

    #[event("performDeployFromSource")]
    fn perform_deploy_from_source_event(
        &self,
        #[indexed] action_id: usize,
        #[indexed] egld_value: &BigUint,
        #[indexed] source_address: &ManagedAddress,
        #[indexed] code_metadata: CodeMetadata,
        #[indexed] gas: u64,
        #[indexed] arguments: &MultiValueManagedVec<ManagedBuffer>,
    );

    #[event("performUpgradeFromSource")]
    fn perform_upgrade_from_source_event(
        &self,
        #[indexed] action_id: usize,
        #[indexed] target_address: &ManagedAddress,
        #[indexed] egld_value: &BigUint,
        #[indexed] source_address: &ManagedAddress,
        #[indexed] code_metadata: CodeMetadata,
        #[indexed] gas: u64,
        #[indexed] arguments: &MultiValueManagedVec<ManagedBuffer>,
    );
}
