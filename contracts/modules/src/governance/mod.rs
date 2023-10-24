multiversx_sc::imports!();

pub mod governance_configurable;
pub mod governance_events;
pub mod governance_proposal;

use governance_proposal::*;

const MAX_GAS_LIMIT_PER_BLOCK: u64 = 600_000_000;
const MIN_AMOUNT_PER_DEPOSIT: u64 = 1;
pub const ALREADY_VOTED_ERR_MSG: &[u8] = b"Already voted for this proposal";
pub const MIN_FEES_REACHED: &[u8] = b"Propose already reached min threshold for fees";
pub const MIN_AMOUNT_NOT_REACHED: &[u8] = b"Minimum amount not reached";

#[multiversx_sc::module]
pub trait GovernanceModule:
    governance_configurable::GovernanceConfigurablePropertiesModule
    + governance_events::GovernanceEventsModule
{
    // endpoints

    /// Used to deposit tokens for "payable" actions.
    /// Funds will be returned if the proposal is defeated.
    /// To keep the logic simple, all tokens have to be deposited at once
    #[payable("*")]
    #[endpoint(depositTokensForProposal)]
    fn deposit_tokens_for_proposal(&self, proposal_id: ProposalId) {
        self.require_caller_not_self();
        self.require_valid_proposal_id(proposal_id);
        require!(
            self.get_proposal_status(proposal_id) == GovernanceProposalStatus::WaitingForFees,
            "Proposal has to be executed or canceled first"
        );
        require!(
            !self.proposal_reached_min_fees(proposal_id),
            MIN_FEES_REACHED
        );
        let additional_fee = self.require_payment_token_governance_token();
        require!(
            additional_fee.amount >= MIN_AMOUNT_PER_DEPOSIT,
            MIN_AMOUNT_NOT_REACHED
        );

        let caller = self.blockchain().get_caller();
        let mut proposal = self.proposals().get(proposal_id);
        proposal.fees.entries.push(FeeEntry {
            depositor_addr: caller.clone(),
            tokens: additional_fee.clone(),
        });
        proposal.fees.total_amount += additional_fee.amount.clone();
        self.proposals().set(proposal_id, &proposal);

        self.user_deposit_event(&caller, proposal_id, &additional_fee);
    }

    // Used to withdraw the tokens after the action was executed or cancelled
    #[endpoint(withdrawGovernanceTokens)]
    fn claim_deposited_tokens(&self, proposal_id: usize) {
        self.require_caller_not_self();
        self.require_valid_proposal_id(proposal_id);
        require!(
            self.get_proposal_status(proposal_id) == GovernanceProposalStatus::WaitingForFees,
            "Cannot claim deposited tokens anymore; Proposal is not in WatingForFees state"
        );

        require!(
            !self.proposal_reached_min_fees(proposal_id),
            MIN_FEES_REACHED
        );

        let caller = self.blockchain().get_caller();
        let mut proposal = self.proposals().get(proposal_id);
        let mut fees_to_send = ManagedVec::<Self::Api, FeeEntry<Self::Api>>::new();
        let mut i = 0;
        while i < proposal.fees.entries.len() {
            if proposal.fees.entries.get(i).depositor_addr == caller {
                fees_to_send.push(proposal.fees.entries.get(i));
                proposal.fees.entries.remove(i);
            } else {
                i += 1;
            }
        }

        for fee_entry in fees_to_send.iter() {
            let payment = fee_entry.tokens.clone();

            self.send().direct_esdt(
                &fee_entry.depositor_addr,
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
            );
            self.user_claim_event(&caller, proposal_id, &fee_entry.tokens);
        }
    }

    /// Propose a list of actions.
    /// A maximum of MAX_GOVERNANCE_PROPOSAL_ACTIONS can be proposed at a time.
    ///
    /// An action has the following format:
    ///     - gas limit for action execution
    ///     - destination address
    ///     - a vector of ESDT transfers, in the form of ManagedVec<EsdTokenPayment>
    ///     - endpoint to be called on the destination
    ///     - a vector of arguments for the endpoint, in the form of ManagedVec<ManagedBuffer>
    ///
    /// Returns the ID of the newly created proposal.
    #[payable("*")]
    #[endpoint]
    fn propose(
        &self,
        description: ManagedBuffer,
        actions: MultiValueEncoded<GovernanceActionAsMultiArg<Self::Api>>,
    ) -> usize {
        self.require_caller_not_self();

        let payment = self.require_payment_token_governance_token();

        require!(
            payment.amount >= self.min_token_balance_for_proposing().get(),
            "Not enough tokens for proposing action"
        );
        require!(!actions.is_empty(), "Proposal has no actions");
        require!(
            actions.len() <= MAX_GOVERNANCE_PROPOSAL_ACTIONS,
            "Exceeded max actions per proposal"
        );

        let mut gov_actions = ArrayVec::new();
        for action in actions {
            let (gas_limit, dest_address, function_name, arguments) = action.into_tuple();
            let gov_action = GovernanceAction {
                gas_limit,
                dest_address,
                function_name,
                arguments,
            };

            require!(
                gas_limit < MAX_GAS_LIMIT_PER_BLOCK,
                "A single action cannot use more than the max gas limit per block"
            );

            gov_actions.push(gov_action);
        }

        require!(
            self.total_gas_needed(&gov_actions) < MAX_GAS_LIMIT_PER_BLOCK,
            "Actions require too much gas to be executed"
        );

        let proposer = self.blockchain().get_caller();
        let fees_entries = ManagedVec::from_single_item(FeeEntry {
            depositor_addr: proposer.clone(),
            tokens: payment.clone(),
        });

        let proposal = GovernanceProposal {
            proposer: proposer.clone(),
            description,
            actions: gov_actions,
            fees: ProposalFees {
                total_amount: payment.amount,
                entries: fees_entries,
            },
        };

        let proposal_id = self.proposals().push(&proposal);
        self.proposal_votes(proposal_id).set(ProposalVotes::new());

        let current_block = self.blockchain().get_block_nonce();
        self.proposal_start_block(proposal_id).set(current_block);

        self.proposal_created_event(proposal_id, &proposer, current_block, &proposal);

        proposal_id
    }

    /// Vote on a proposal by depositing any amount of governance tokens
    /// These tokens will be locked until the proposal is executed or cancelled.
    #[payable("*")]
    #[endpoint]
    fn vote(&self, proposal_id: usize, vote: VoteType) {
        self.require_caller_not_self();

        let payment = self.require_payment_token_governance_token();
        self.require_valid_proposal_id(proposal_id);
        require!(
            self.get_proposal_status(proposal_id) == GovernanceProposalStatus::Active,
            "Proposal is not active"
        );

        let voter = self.blockchain().get_caller();
        let new_user = self.user_voted_proposals(&voter).insert(proposal_id);
        require!(new_user, ALREADY_VOTED_ERR_MSG);

        match vote {
            VoteType::UpVote => {
                self.proposal_votes(proposal_id).update(|total_votes| {
                    total_votes.up_votes += &payment.amount.clone();
                });
                self.up_vote_cast_event(&voter, proposal_id, &payment.amount);
            },
            VoteType::DownVote => {
                self.proposal_votes(proposal_id).update(|total_votes| {
                    total_votes.down_votes += &payment.amount.clone();
                });
                self.down_vote_cast_event(&voter, proposal_id, &payment.amount);
            },
            VoteType::DownVetoVote => {
                self.proposal_votes(proposal_id).update(|total_votes| {
                    total_votes.down_veto_votes += &payment.amount.clone();
                });
                self.down_veto_vote_cast_event(&voter, proposal_id, &payment.amount);
            },
            VoteType::AbstainVote => {
                self.proposal_votes(proposal_id).update(|total_votes| {
                    total_votes.abstain_votes += &payment.amount.clone();
                });
                self.abstain_vote_cast_event(&voter, proposal_id, &payment.amount);
            },
        }
    }

    /// Queue a proposal for execution.
    /// This can be done only if the proposal has reached the quorum.
    /// A proposal is considered successful and ready for queing if
    /// total_votes - total_downvotes >= quorum
    #[endpoint]
    fn queue(&self, proposal_id: usize) {
        self.require_caller_not_self();

        require!(
            self.get_proposal_status(proposal_id) == GovernanceProposalStatus::Succeeded,
            "Can only queue succeeded proposals"
        );

        let current_block = self.blockchain().get_block_nonce();
        self.proposal_queue_block(proposal_id).set(current_block);

        self.proposal_queued_event(proposal_id, current_block);
    }

    /// Execute a previously queued proposal.
    /// This will clear the proposal and unlock the governance tokens.
    /// Said tokens can then be withdrawn and used to vote/downvote other proposals.
    #[endpoint]
    fn execute(&self, proposal_id: usize) {
        self.require_caller_not_self();

        require!(
            self.get_proposal_status(proposal_id) == GovernanceProposalStatus::Queued,
            "Can only execute queued proposals"
        );

        let current_block = self.blockchain().get_block_nonce();
        let lock_blocks = self.lock_time_after_voting_ends_in_blocks().get();

        let lock_start = self.proposal_queue_block(proposal_id).get();
        let lock_end = lock_start + lock_blocks;

        require!(
            current_block >= lock_end,
            "Proposal is in timelock status. Try again later"
        );

        let proposal = self.proposals().get(proposal_id);
        let total_gas_needed = self.total_gas_needed(&proposal.actions);
        let gas_left = self.blockchain().get_gas_left();

        require!(
            gas_left > total_gas_needed,
            "Not enough gas to execute all proposals"
        );

        self.clear_proposal(proposal_id);

        for action in proposal.actions {
            let mut contract_call = self
                .send()
                .contract_call::<()>(action.dest_address, action.function_name)
                .with_gas_limit(action.gas_limit);

            for arg in &action.arguments {
                contract_call.push_raw_argument(arg);
            }

            contract_call.transfer_execute();
        }

        self.proposal_executed_event(proposal_id);
    }

    /// Cancel a proposed action. This can be done:
    /// - by the proposer, at any time
    /// - by anyone, if the proposal was defeated
    #[endpoint]
    fn cancel(&self, proposal_id: usize) {
        self.require_caller_not_self();

        match self.get_proposal_status(proposal_id) {
            GovernanceProposalStatus::None => {
                sc_panic!("Proposal does not exist");
            },
            GovernanceProposalStatus::Pending => {
                let proposal = self.proposals().get(proposal_id);
                let caller = self.blockchain().get_caller();

                require!(
                    caller == proposal.proposer,
                    "Only original proposer may cancel a pending proposal"
                );
            },
            GovernanceProposalStatus::Defeated => {},
            GovernanceProposalStatus::WaitingForFees => {
                self.refund_payments(proposal_id);
            },
            _ => {
                sc_panic!("Action may not be cancelled");
            },
        }

        self.clear_proposal(proposal_id);
        self.proposal_canceled_event(proposal_id);
    }

    // views

    #[view(getProposalStatus)]
    fn get_proposal_status(&self, proposal_id: usize) -> GovernanceProposalStatus {
        if !self.proposal_exists(proposal_id) {
            return GovernanceProposalStatus::None;
        }

        let queue_block = self.proposal_queue_block(proposal_id).get();
        if queue_block > 0 {
            return GovernanceProposalStatus::Queued;
        }

        let current_block = self.blockchain().get_block_nonce();
        let proposal_block = self.proposal_start_block(proposal_id).get();
        let voting_delay = self.voting_delay_in_blocks().get();
        let voting_period = self.voting_period_in_blocks().get();

        let voting_start = proposal_block + voting_delay;
        let voting_end = voting_start + voting_period;

        if current_block < voting_start {
            return GovernanceProposalStatus::Pending;
        }
        if current_block >= voting_start && current_block < voting_end {
            return GovernanceProposalStatus::Active;
        }

        if self.quorum_and_vote_reached(proposal_id) {
            GovernanceProposalStatus::Succeeded
        } else {
            GovernanceProposalStatus::Defeated
        }
    }

    fn quorum_and_vote_reached(&self, proposal_id: ProposalId) -> bool {
        let proposal_votes = self.proposal_votes(proposal_id).get();
        let total_votes = proposal_votes.get_total_votes();
        let total_up_votes = proposal_votes.up_votes;
        let total_down_votes = proposal_votes.down_votes;
        let total_down_veto_votes = proposal_votes.down_veto_votes;
        let third_total_votes = &total_votes / 3u64;
        let quorum = self.quorum().get();

        sc_print!("Total votes = {} quorum = {}", total_votes, quorum);
        if total_down_veto_votes > third_total_votes {
            false
        } else {
            total_votes >= quorum && total_up_votes > (total_down_votes + total_down_veto_votes)
        }
    }

    #[view(getProposer)]
    fn get_proposer(&self, proposal_id: usize) -> OptionalValue<ManagedAddress> {
        if !self.proposal_exists(proposal_id) {
            OptionalValue::None
        } else {
            OptionalValue::Some(self.proposals().get(proposal_id).proposer)
        }
    }

    #[view(getProposalDescription)]
    fn get_proposal_description(&self, proposal_id: usize) -> OptionalValue<ManagedBuffer> {
        if !self.proposal_exists(proposal_id) {
            OptionalValue::None
        } else {
            OptionalValue::Some(self.proposals().get(proposal_id).description)
        }
    }

    #[view(getProposalActions)]
    fn get_proposal_actions(
        &self,
        proposal_id: usize,
    ) -> MultiValueEncoded<GovernanceActionAsMultiArg<Self::Api>> {
        if !self.proposal_exists(proposal_id) {
            return MultiValueEncoded::new();
        }

        let actions = self.proposals().get(proposal_id).actions;
        let mut actions_as_multiarg = MultiValueEncoded::new();

        for action in actions {
            actions_as_multiarg.push(action.into_multiarg());
        }

        actions_as_multiarg
    }

    // private

    fn refund_payments(&self, proposal_id: ProposalId) {
        let payments = self.proposals().get(proposal_id).fees;

        for fee_entry in payments.entries.iter() {
            let payment = fee_entry.tokens;
            self.send().direct_esdt(
                &fee_entry.depositor_addr,
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
            );
        }
    }

    fn require_payment_token_governance_token(&self) -> EsdtTokenPayment {
        let payment = self.call_value().single_esdt();
        require!(
            payment.token_identifier == self.governance_token_id().get(),
            "Only Governance token accepted as payment"
        );
        payment
    }

    fn require_valid_proposal_id(&self, proposal_id: usize) {
        require!(
            self.is_valid_proposal_id(proposal_id),
            "Invalid proposal ID"
        );
    }

    fn require_caller_not_self(&self) {
        let caller = self.blockchain().get_caller();
        let sc_address = self.blockchain().get_sc_address();

        require!(
            caller != sc_address,
            "Cannot call this endpoint through proposed action"
        );
    }

    fn is_valid_proposal_id(&self, proposal_id: usize) -> bool {
        proposal_id >= 1 && proposal_id <= self.proposals().len()
    }

    fn proposal_reached_min_fees(&self, proposal_id: ProposalId) -> bool {
        let accumulated_fees = self.proposals().get(proposal_id).fees.total_amount;
        let min_fees = self.min_fee_for_propose().get();
        accumulated_fees >= min_fees
    }

    fn proposal_exists(&self, proposal_id: usize) -> bool {
        self.is_valid_proposal_id(proposal_id) && !self.proposals().item_is_empty(proposal_id)
    }

    fn total_gas_needed(
        &self,
        actions: &ArrayVec<GovernanceAction<Self::Api>, MAX_GOVERNANCE_PROPOSAL_ACTIONS>,
    ) -> u64 {
        let mut total = 0;
        for action in actions {
            total += action.gas_limit;
        }

        total
    }

    /// specific votes/downvotes are not cleared,
    /// as they're used for reclaim tokens logic and cleared one by one
    fn clear_proposal(&self, proposal_id: usize) {
        self.proposals().clear_entry(proposal_id);
        self.proposal_start_block(proposal_id).clear();
        self.proposal_queue_block(proposal_id).clear();

        self.total_votes(proposal_id).clear();
        self.total_downvotes(proposal_id).clear();
    }

    // storage - general

    #[storage_mapper("governance:proposals")]
    fn proposals(&self) -> VecMapper<GovernanceProposal<Self::Api>>;

    /// Not stored under "proposals", as that would require deserializing the whole struct
    #[storage_mapper("governance:proposalStartBlock")]
    fn proposal_start_block(&self, proposal_id: usize) -> SingleValueMapper<u64>;

    #[storage_mapper("governance:proposalQueueBlock")]
    fn proposal_queue_block(&self, proposal_id: usize) -> SingleValueMapper<u64>;

    #[storage_mapper("governance:userVotedProposals")]
    fn user_voted_proposals(&self, user: &ManagedAddress) -> UnorderedSetMapper<ProposalId>;

    #[view(getProposalVotes)]
    #[storage_mapper("proposalVotes")]
    fn proposal_votes(
        &self,
        proposal_id: ProposalId,
    ) -> SingleValueMapper<ProposalVotes<Self::Api>>;

    #[view(getTotalVotes)]
    #[storage_mapper("governance:totalVotes")]
    fn total_votes(&self, proposal_id: usize) -> SingleValueMapper<BigUint>;

    #[view(getTotalDownvotes)]
    #[storage_mapper("governance:totalDownvotes")]
    fn total_downvotes(&self, proposal_id: usize) -> SingleValueMapper<BigUint>;
}
