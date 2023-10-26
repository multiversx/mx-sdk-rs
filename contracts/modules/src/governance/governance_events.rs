multiversx_sc::imports!();

use super::governance_proposal::GovernanceProposal;
use crate::governance::ProposalId;

#[multiversx_sc::module]
pub trait GovernanceEventsModule {
    #[event("proposalCreated")]
    fn proposal_created_event(
        &self,
        #[indexed] proposal_id: usize,
        #[indexed] proposer: &ManagedAddress,
        #[indexed] start_block: u64,
        proposal: &GovernanceProposal<CurrentApi>,
    );

    #[event("upVoteCast")]
    fn up_vote_cast_event(
        &self,
        #[indexed] up_voter: &ManagedAddress,
        #[indexed] proposal_id: ProposalId,
        nr_votes: &BaseBigUint,
    );

    #[event("downVoteCast")]
    fn down_vote_cast_event(
        &self,
        #[indexed] down_voter: &ManagedAddress,
        #[indexed] proposal_id: ProposalId,
        nr_downvotes: &BaseBigUint,
    );

    #[event("downVetoVoteCast")]
    fn down_veto_vote_cast_event(
        &self,
        #[indexed] down_veto_voter: &ManagedAddress,
        #[indexed] proposal_id: ProposalId,
        nr_downvotes: &BaseBigUint,
    );

    #[event("abstainVoteCast")]
    fn abstain_vote_cast_event(
        &self,
        #[indexed] abstain_voter: &ManagedAddress,
        #[indexed] proposal_id: ProposalId,
        nr_downvotes: &BaseBigUint,
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
        #[indexed] proposal_id: ProposalId,
        payment: &EsdtTokenPayment<CurrentApi>,
    );

    #[event("userClaimDepositedTokens")]
    fn user_claim_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] proposal_id: ProposalId,
        payment: &EsdtTokenPayment<CurrentApi>,
    );
}
