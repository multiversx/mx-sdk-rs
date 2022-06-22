elrond_wasm::imports!();

pub mod governance_configurable;
pub mod governance_events;
pub mod governance_proposal;

use governance_proposal::*;

const MAX_GAS_LIMIT_PER_BLOCK: u64 = 1_500_000_000;

#[elrond_wasm::module]
pub trait GovernanceModule:
    governance_configurable::GovernanceConfigurablePropertiesModule
    + governance_events::GovernanceEventsModule
{
    // endpoints

    // Used to deposit tokens for "payable" actions
    // There is no "withdraw" functionality
    // Funds can only be retrived through an action
    #[payable("*")]
    #[endpoint(depositTokensForAction)]
    fn deposit_tokens_for_action(&self) {
        let payments = self.call_value().all_esdt_transfers();
        let caller = self.blockchain().get_caller();

        self.user_deposit_event(&caller, &payments);
    }

    // Used to withdraw the tokens after the action was executed or cancelled
    #[endpoint(withdrawGovernanceTokens)]
    fn withdraw_governance_tokens(&self, proposal_id: usize) {
        self.require_valid_proposal_id(proposal_id);
        require!(
            self.get_proposal_status(proposal_id) == GovernanceProposalStatus::None,
            "Proposal has to be executed or canceled first"
        );

        let caller = self.blockchain().get_caller();
        let governance_token_id = self.governance_token_id().get();

        let votes_mapper = self.votes(proposal_id, &caller);
        let downvotes_mapper = self.downvotes(proposal_id, &caller);

        let nr_votes_tokens = votes_mapper.get();
        let nr_downvotes_tokens = downvotes_mapper.get();
        let total_tokens = nr_votes_tokens + nr_downvotes_tokens;

        if total_tokens > 0 {
            votes_mapper.clear();
            downvotes_mapper.clear();

            self.send()
                .direct_esdt(&caller, &governance_token_id, 0, &total_tokens);
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
            let (gas_limit, dest_address, payments, function_name, arguments) = action.into_tuple();
            let gov_action = GovernanceAction {
                gas_limit,
                dest_address,
                payments,
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
        let current_block = self.blockchain().get_block_nonce();
        let proposal_id = self.proposals().len() + 1;

        self.proposal_created_event(
            proposal_id,
            &proposer,
            current_block,
            &description,
            &gov_actions,
        );

        self.proposal_start_block(proposal_id).set(&current_block);
        self.total_votes(proposal_id).set(&payment.amount);
        self.votes(proposal_id, &proposer).set(&payment.amount);

        let proposal = GovernanceProposal {
            proposer,
            description,
            actions: gov_actions,
        };
        let _ = self.proposals().push(&proposal);

        proposal_id
    }

    /// Vote on a proposal by depositing any amount of governance tokens
    /// These tokens will be locked until the proposal is executed or cancelled.
    #[payable("*")]
    #[endpoint]
    fn vote(&self, proposal_id: usize) {
        let payment = self.require_payment_token_governance_token();
        self.require_valid_proposal_id(proposal_id);
        require!(
            self.get_proposal_status(proposal_id) == GovernanceProposalStatus::Active,
            "Proposal is not active"
        );

        let voter = self.blockchain().get_caller();
        self.vote_cast_event(&voter, proposal_id, &payment.amount);

        self.total_votes(proposal_id)
            .update(|total_votes| *total_votes += &payment.amount);
        self.votes(proposal_id, &voter)
            .update(|nr_votes| *nr_votes += &payment.amount);
    }

    /// Downvote a proposal by depositing any amount of governance tokens.
    /// These tokens will be locked until the proposal is executed or cancelled.
    #[payable("*")]
    #[endpoint]
    fn downvote(&self, proposal_id: usize) {
        let payment = self.require_payment_token_governance_token();
        self.require_valid_proposal_id(proposal_id);
        require!(
            self.get_proposal_status(proposal_id) == GovernanceProposalStatus::Active,
            "Proposal is not active"
        );

        let downvoter = self.blockchain().get_caller();
        self.downvote_cast_event(&downvoter, proposal_id, &payment.amount);

        self.total_downvotes(proposal_id)
            .update(|total_downvotes| *total_downvotes += &payment.amount);
        self.downvotes(proposal_id, &downvoter)
            .update(|nr_downvotes| *nr_downvotes += &payment.amount);
    }

    /// Queue a proposal for execution.
    /// This can be done only if the proposal has reached the quorum.
    /// A proposal is considered successful and ready for queing if
    /// total_votes - total_downvotes >= quorum
    #[endpoint]
    fn queue(&self, proposal_id: usize) {
        require!(
            self.get_proposal_status(proposal_id) == GovernanceProposalStatus::Succeeded,
            "Can only queue succeeded proposals"
        );

        let current_block = self.blockchain().get_block_nonce();
        self.proposal_queue_block(proposal_id).set(&current_block);

        self.proposal_queued_event(proposal_id, current_block);
    }

    /// Execute a previously queued proposal.
    /// This will clear the proposal and unlock the governance tokens.
    /// Said tokens can then be withdrawn and used to vote/downvote other proposals.
    #[endpoint]
    fn execute(&self, proposal_id: usize) {
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

        for action in proposal.actions {
            let mut contract_call = self
                .send()
                .contract_call::<()>(action.dest_address, action.function_name)
                .with_gas_limit(action.gas_limit);

            if !action.payments.is_empty() {
                contract_call = contract_call.with_multi_token_transfer(action.payments);
            }

            for arg in &action.arguments {
                contract_call.push_arg_managed_buffer(arg);
            }

            contract_call.transfer_execute();
        }

        self.clear_proposal(proposal_id);

        self.proposal_executed_event(proposal_id);
    }

    /// Cancel a proposed action. This can be done:
    /// - by the proposer, at any time
    /// - by anyone, if the proposal was defeated
    #[endpoint]
    fn cancel(&self, proposal_id: usize) {
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

        let total_votes = self.total_votes(proposal_id).get();
        let total_downvotes = self.total_downvotes(proposal_id).get();
        let quorum = self.quorum().get();

        if total_votes > total_downvotes && total_votes - total_downvotes >= quorum {
            GovernanceProposalStatus::Succeeded
        } else {
            GovernanceProposalStatus::Defeated
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

    fn is_valid_proposal_id(&self, proposal_id: usize) -> bool {
        proposal_id >= 1 && proposal_id <= self.proposals().len()
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

    #[storage_mapper("governance:votes")]
    fn votes(&self, proposal_id: usize, voter: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("governance:downvotes")]
    fn downvotes(
        &self,
        proposal_id: usize,
        downvoter: &ManagedAddress,
    ) -> SingleValueMapper<BigUint>;

    #[view(getTotalVotes)]
    #[storage_mapper("governance:totalVotes")]
    fn total_votes(&self, proposal_id: usize) -> SingleValueMapper<BigUint>;

    #[view(getTotalDownvotes)]
    #[storage_mapper("governance:totalDownvotes")]
    fn total_downvotes(&self, proposal_id: usize) -> SingleValueMapper<BigUint>;
}
