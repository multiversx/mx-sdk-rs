#![no_std]

elrond_wasm::imports!();

pub mod governance_configurable;

pub mod governance_proposal;
use governance_proposal::*;

const MAX_GAS_LIMIT_PER_BLOCK: u64 = 1_500_000_000;

#[elrond_wasm::module]
pub trait GovernanceModule:
    governance_configurable::GovernanceConfigurablePropertiesModule
{
    // endpoints

    // Used to deposit tokens for "payable" actions
    // There is no "withdraw" functionality
    // Funds can only be retrived through an action
    #[payable("*")]
    #[endpoint(depositTokensForAction)]
    fn deposit_tokens_for_action(
        &self,
        #[payment_token] payment_token: TokenIdentifier,
        #[payment_nonce] payment_nonce: u64,
        #[payment_amount] payment_amount: BigUint,
    ) {
        let caller = self.blockchain().get_caller();

        self.user_deposit_event(&caller, &payment_token, payment_nonce, &payment_amount);
    }

    // Used to withdraw the tokens after the action was executed or cancelled
    #[endpoint(withdrawGovernanceTokens)]
    fn withdraw_governance_tokens(&self, proposal_id: usize) -> SCResult<()> {
        self.require_valid_proposal_id(proposal_id)?;
        require!(
            self.get_proposal_status(proposal_id) == GovernanceProposalStatus::None,
            "Proposal has to be executed or canceled first"
        );

        let caller = self.blockchain().get_caller();
        let governance_token_id = self.governance_token_id().get();
        let nr_votes_tokens = self.votes(proposal_id).get(&caller).unwrap_or_default();
        let nr_downvotes_tokens = self.downvotes(proposal_id).get(&caller).unwrap_or_default();
        let total_tokens = nr_votes_tokens + nr_downvotes_tokens;

        if total_tokens > 0 {
            self.votes(proposal_id).remove(&caller);
            self.downvotes(proposal_id).remove(&caller);

            self.send()
                .direct(&caller, &governance_token_id, 0, &total_tokens, &[]);
        }

        Ok(())
    }

    #[payable("*")]
    #[endpoint]
    fn propose(
        &self,
        #[payment_amount] payment_amount: BigUint,
        description: BoxedBytes,
        #[var_args] actions: VarArgs<GovernanceActionAsMultiArg<Self::Api>>,
    ) -> SCResult<usize> {
        self.require_payment_token_governance_token()?;
        require!(
            payment_amount >= self.min_token_balance_for_proposing().get(),
            "Not enough tokens for proposing action"
        );
        require!(!actions.is_empty(), "Proposal has no actions");
        require!(
            actions.len() <= self.max_actions_per_proposal().get(),
            "Exceeded max actions per proposal"
        );

        let mut gov_actions = Vec::with_capacity(actions.len());
        for action in actions.into_vec() {
            let (gas_limit, dest_address, token_id, token_nonce, amount, function_name, arguments) =
                action.into_tuple();
            let gov_action = GovernanceAction {
                gas_limit,
                dest_address,
                token_id,
                token_nonce,
                amount,
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

        let proposal = GovernanceProposal {
            proposer: proposer.clone(),
            description,
            actions: gov_actions,
        };
        let _ = self.proposals().push(&proposal);

        self.proposal_start_block(proposal_id).set(&current_block);

        self.total_votes(proposal_id).set(&payment_amount);
        self.votes(proposal_id).insert(proposer, payment_amount);

        Ok(proposal_id)
    }

    #[payable("*")]
    #[endpoint]
    fn vote(&self, #[payment_amount] payment_amount: BigUint, proposal_id: usize) -> SCResult<()> {
        self.require_payment_token_governance_token()?;
        self.require_valid_proposal_id(proposal_id)?;
        require!(
            self.get_proposal_status(proposal_id) == GovernanceProposalStatus::Active,
            "Proposal is not active"
        );

        let voter = self.blockchain().get_caller();

        self.vote_cast_event(&voter, proposal_id, &payment_amount);

        self.total_votes(proposal_id)
            .update(|total_votes| *total_votes += &payment_amount);
        self.votes(proposal_id)
            .entry(voter)
            .and_modify(|nr_votes| *nr_votes += &payment_amount)
            .or_insert(payment_amount);

        Ok(())
    }

    #[payable("*")]
    #[endpoint]
    fn downvote(
        &self,
        #[payment_amount] payment_amount: BigUint,
        proposal_id: usize,
    ) -> SCResult<()> {
        self.require_payment_token_governance_token()?;
        self.require_valid_proposal_id(proposal_id)?;
        require!(
            self.get_proposal_status(proposal_id) == GovernanceProposalStatus::Active,
            "Proposal is not active"
        );

        let downvoter = self.blockchain().get_caller();

        self.downvote_cast_event(&downvoter, proposal_id, &payment_amount);

        self.total_downvotes(proposal_id)
            .update(|total_downvotes| *total_downvotes += &payment_amount);
        self.downvotes(proposal_id)
            .entry(downvoter)
            .and_modify(|nr_downvotes| *nr_downvotes += &payment_amount)
            .or_insert(payment_amount);

        Ok(())
    }

    #[endpoint]
    fn queue(&self, proposal_id: usize) -> SCResult<()> {
        require!(
            self.get_proposal_status(proposal_id) == GovernanceProposalStatus::Succeeded,
            "Can only queue succeeded proposals"
        );

        let current_block = self.blockchain().get_block_nonce();
        self.proposal_queue_block(proposal_id).set(&current_block);

        self.proposal_queued_event(proposal_id, current_block);

        Ok(())
    }

    #[endpoint]
    fn execute(&self, proposal_id: usize) -> SCResult<()> {
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

            if action.amount > 0 {
                contract_call = contract_call.add_token_transfer(
                    action.token_id,
                    action.token_nonce,
                    action.amount,
                );
            }

            for arg in action.arguments {
                contract_call.push_argument_raw_bytes(arg.as_slice());
            }

            contract_call.transfer_execute();
        }

        self.clear_proposal(proposal_id);

        self.proposal_executed_event(proposal_id);

        Ok(())
    }

    #[endpoint]
    fn cancel(&self, proposal_id: usize) -> SCResult<()> {
        match self.get_proposal_status(proposal_id) {
            GovernanceProposalStatus::None => {
                return sc_error!("Proposal does not exist");
            },
            GovernanceProposalStatus::Defeated => {},
            _ => {
                let proposal = self.proposals().get(proposal_id);
                let caller = self.blockchain().get_caller();

                require!(
                    caller == proposal.proposer,
                    "Only original proposer may cancel a non-defeated proposal"
                );
            },
        }

        self.clear_proposal(proposal_id);

        self.proposal_canceled_event(proposal_id);

        Ok(())
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
    fn get_proposer(&self, proposal_id: usize) -> OptionalArg<ManagedAddress> {
        if !self.proposal_exists(proposal_id) {
            OptionalArg::None
        } else {
            OptionalArg::Some(self.proposals().get(proposal_id).proposer)
        }
    }

    #[view(getProposalDescription)]
    fn get_proposal_description(&self, proposal_id: usize) -> OptionalArg<BoxedBytes> {
        if !self.proposal_exists(proposal_id) {
            OptionalArg::None
        } else {
            OptionalArg::Some(self.proposals().get(proposal_id).description)
        }
    }

    #[view(getProposalActions)]
    fn get_proposal_actions(
        &self,
        proposal_id: usize,
    ) -> MultiResultVec<GovernanceActionAsMultiArg<Self::Api>> {
        if !self.proposal_exists(proposal_id) {
            return MultiResultVec::new();
        }

        let actions = self.proposals().get(proposal_id).actions;
        let mut actions_as_multiarg = Vec::with_capacity(actions.len());

        for action in actions {
            actions_as_multiarg.push(action.into_multiarg());
        }

        actions_as_multiarg.into()
    }

    // private

    fn require_payment_token_governance_token(&self) -> SCResult<()> {
        require!(
            self.call_value().token() == self.governance_token_id().get(),
            "Only Governance token accepted as payment"
        );
        Ok(())
    }

    fn require_valid_proposal_id(&self, proposal_id: usize) -> SCResult<()> {
        require!(
            self.is_valid_proposal_id(proposal_id),
            "Invalid proposal ID"
        );
        Ok(())
    }

    fn is_valid_proposal_id(&self, proposal_id: usize) -> bool {
        proposal_id >= 1 && proposal_id <= self.proposals().len()
    }

    fn proposal_exists(&self, proposal_id: usize) -> bool {
        self.is_valid_proposal_id(proposal_id) && !self.proposals().item_is_empty(proposal_id)
    }

    fn total_gas_needed(&self, actions: &[GovernanceAction<Self::Api>]) -> u64 {
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

    // events

    #[event("proposalCreated")]
    fn proposal_created_event(
        &self,
        #[indexed] proposal_id: usize,
        #[indexed] proposer: &ManagedAddress,
        #[indexed] start_block: u64,
        #[indexed] description: &BoxedBytes,
        actions: &[GovernanceAction<Self::Api>],
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
        #[indexed] token_id: &TokenIdentifier,
        #[indexed] token_nonce: u64,
        amount: &BigUint,
    );

    // storage - general

    #[storage_mapper("governance:proposals")]
    fn proposals(&self) -> VecMapper<GovernanceProposal<Self::Api>>;

    /// Not stored under "proposals", as that would require deserializing the whole struct
    #[storage_mapper("governance:proposalStartBlock")]
    fn proposal_start_block(&self, proposal_id: usize) -> SingleValueMapper<u64>;

    #[storage_mapper("governance:proposalQueueBlock")]
    fn proposal_queue_block(&self, proposal_id: usize) -> SingleValueMapper<u64>;

    #[storage_mapper("governance:votes")]
    fn votes(&self, proposal_id: usize) -> MapMapper<ManagedAddress, BigUint>;

    #[storage_mapper("governance:downvotes")]
    fn downvotes(&self, proposal_id: usize) -> MapMapper<ManagedAddress, BigUint>;

    /// Could be calculated by iterating over the "votes" mapper, but that costs a lot of gas
    #[view(getTotalVotes)]
    #[storage_mapper("governance:totalVotes")]
    fn total_votes(&self, proposal_id: usize) -> SingleValueMapper<BigUint>;

    /// Could be calculated by iterating over the "downvotes" mapper, but that costs a lot of gas
    #[view(getTotalDownvotes)]
    #[storage_mapper("governance:totalDownvotes")]
    fn total_downvotes(&self, proposal_id: usize) -> SingleValueMapper<BigUint>;
}
