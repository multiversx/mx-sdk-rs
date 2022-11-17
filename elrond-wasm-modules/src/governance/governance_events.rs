elrond_wasm::imports!();

use super::governance_proposal::{GovernanceAction, MAX_GOVERNANCE_PROPOSAL_ACTIONS};

#[elrond_wasm::module]
pub trait GovernanceEventsModule {
    #[event("proposalCreated")]
    fn proposal_created_event(
        &self,
        #[indexed] proposal_id: usize,
        #[indexed] proposer: &ManagedAddress,
        #[indexed] start_block: u64,
        #[indexed] description: &ManagedBuffer,
        actions: &ArrayVec<GovernanceAction<Self::Api>, MAX_GOVERNANCE_PROPOSAL_ACTIONS>,
    );

    #[event("voteCast")]
    fn vote_cast_event(
        &self,
        #[indexed] voter: &ManagedAddress,
        #[indexed] proposal_id: usize,
        nr_votes: &BigUint,
    );

    #[event("downvoteCast")]
    fn downvote_cast_event(
        &self,
        #[indexed] downvoter: &ManagedAddress,
        #[indexed] proposal_id: usize,
        nr_downvotes: &BigUint,
    );

    #[event("proposalCanceled")]
    fn proposal_canceled_event(&self, #[indexed] proposal_id: usize);

    #[event("proposalQueued")]
    fn proposal_queued_event(&self, #[indexed] proposal_id: usize, #[indexed] queued_block: u64);

    #[event("proposalExecuted")]
    fn proposal_executed_event(&self, #[indexed] proposal_id: usize);

    #[event("userDeposit")]
    fn user_deposit_event(
        &self,
        #[indexed] address: &ManagedAddress,
        payments: &ManagedVec<EsdtTokenPayment<Self::Api>>,
    );
}
