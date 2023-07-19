multiversx_sc::imports!();

/// # MultiversX smart contract module - Governance
///
/// This is a standard smart contract module, that when added to a smart contract offers governance features:
/// - proposing actions
/// - voting/downvoting a certain proposal
/// - after a voting period, either putting the action in a queue (if it reached quorum), or canceling
///
/// Voting can only be done by depositing a certain token, decided upon first time setup.  
///
/// The module provides the following configurable parameters:  
/// - `quorum` - the minimum number of (`votes` minus `downvotes`) at the end of voting period  
/// - `minTokenBalanceForProposing` - Minimum numbers of tokens the proposer has to deposit. These automatically count as `votes` as well  
/// - `maxActionsPerProposal` - Maximum number of actions (transfers and/or smart contract calls) that a proposal may have  
/// - `votingDelayInBlocks` - Number of blocks to wait after a block is proposed before being able to vote/downvote that proposal
/// - `votingPeriodInBlocks` - Number of blocks the voting period lasts (voting delay does not count towards this)  
/// - `lockTimeAfterVotingEndsInBlocks` - Number of blocks to wait before a successful proposal can be executed  
///
/// The module also provides events for most actions that happen:
/// - `proposalCreated` - triggers when a proposal is created. Also provoides all the relevant information, like proposer, actions etc.  
/// - `voteCast` - user voted on a proposal  
/// - `downvoteCast` - user downvoted a proposal  
/// - `proposalCanceled`, `proposalQueued` and `proposalExecuted` - provides the ID of the specific proposal  
/// - `userDeposit` - a user deposited some tokens needed for a future payable action  
///
/// Please note that although the main contract can modify the module's storage directly, it is not recommended to do so,
/// as that defeats the whole purpose of having governance. These parameters should only be modified through actions.
///
#[multiversx_sc::module]
pub trait GovernanceConfigurablePropertiesModule {
    // endpoints - owner-only

    /// The module can't protect its storage from the main SC, so it's the developers responsibility
    /// to not modify parameters manually
    fn init_governance_module(
        &self,
        governance_token_id: TokenIdentifier,
        quorum: BigUint,
        min_token_balance_for_proposal: BigUint,
        voting_delay_in_blocks: u64,
        voting_period_in_blocks: u64,
        lock_time_after_voting_ends_in_blocks: u64,
    ) {
        require!(
            governance_token_id.is_valid_esdt_identifier(),
            "Invalid ESDT token ID provided for governance_token_id"
        );

        self.governance_token_id()
            .set_if_empty(&governance_token_id);

        self.try_change_quorum(quorum);
        self.try_change_min_token_balance_for_proposing(min_token_balance_for_proposal);
        self.try_change_voting_delay_in_blocks(voting_delay_in_blocks);
        self.try_change_voting_period_in_blocks(voting_period_in_blocks);
        self.try_change_lock_time_after_voting_ends_in_blocks(
            lock_time_after_voting_ends_in_blocks,
        );
    }

    // endpoints - these can only be called by the SC itself.
    // i.e. only by proposing and executing an action with the SC as dest and the respective func name

    #[endpoint(changeQuorum)]
    fn change_quorum(&self, new_value: BigUint) {
        self.require_caller_self();

        self.try_change_quorum(new_value);
    }

    #[endpoint(changeMinTokenBalanceForProposing)]
    fn change_min_token_balance_for_proposing(&self, new_value: BigUint) {
        self.require_caller_self();

        self.try_change_min_token_balance_for_proposing(new_value);
    }

    #[endpoint(changeVotingDelayInBlocks)]
    fn change_voting_delay_in_blocks(&self, new_value: u64) {
        self.require_caller_self();

        self.try_change_voting_delay_in_blocks(new_value);
    }

    #[endpoint(changeVotingPeriodInBlocks)]
    fn change_voting_period_in_blocks(&self, new_value: u64) {
        self.require_caller_self();

        self.try_change_voting_period_in_blocks(new_value);
    }

    #[endpoint(changeLockTimeAfterVotingEndsInBlocks)]
    fn change_lock_time_after_voting_ends_in_blocks(&self, new_value: u64) {
        self.require_caller_self();

        self.try_change_lock_time_after_voting_ends_in_blocks(new_value);
    }

    // private

    fn require_caller_self(&self) {
        let caller = self.blockchain().get_caller();
        let sc_address = self.blockchain().get_sc_address();

        require!(
            caller == sc_address,
            "Only the SC itself may call this function"
        );
    }

    fn try_change_quorum(&self, new_value: BigUint) {
        require!(new_value != 0, "Quorum can't be set to 0");

        self.quorum().set(&new_value);
    }

    fn try_change_min_token_balance_for_proposing(&self, new_value: BigUint) {
        require!(
            new_value != 0,
            "Min token balance for proposing can't be set to 0"
        );

        self.min_token_balance_for_proposing().set(&new_value);
    }

    fn try_change_voting_delay_in_blocks(&self, new_value: u64) {
        require!(new_value != 0, "Voting delay in blocks can't be set to 0");

        self.voting_delay_in_blocks().set(new_value);
    }

    fn try_change_voting_period_in_blocks(&self, new_value: u64) {
        require!(
            new_value != 0,
            "Voting period (in blocks) can't be set to 0"
        );

        self.voting_period_in_blocks().set(new_value);
    }

    fn try_change_lock_time_after_voting_ends_in_blocks(&self, new_value: u64) {
        require!(
            new_value != 0,
            "Lock time after voting ends (in blocks) can't be set to 0"
        );

        self.lock_time_after_voting_ends_in_blocks().set(new_value);
    }

    // storage - fixed parameters

    #[view(getGovernanceTokenId)]
    #[storage_mapper("governance:governanceTokenId")]
    fn governance_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    // storage - configurable parameters

    #[view(getQuorum)]
    #[storage_mapper("governance:quorum")]
    fn quorum(&self) -> SingleValueMapper<BigUint>;

    #[view(getMinFeeForPropose)]
    #[storage_mapper("minFeeForPropose")]
    fn min_fee_for_propose(&self) -> SingleValueMapper<BigUint>;

    #[view(getMinTokenBalanceForProposing)]
    #[storage_mapper("governance:minTokenBalanceForProposing")]
    fn min_token_balance_for_proposing(&self) -> SingleValueMapper<BigUint>;

    #[view(getVotingDelayInBlocks)]
    #[storage_mapper("governance:votingDelayInBlocks")]
    fn voting_delay_in_blocks(&self) -> SingleValueMapper<u64>;

    #[view(getVotingPeriodInBlocks)]
    #[storage_mapper("governance:votingPeriodInBlocks")]
    fn voting_period_in_blocks(&self) -> SingleValueMapper<u64>;

    #[view(getLockTimeAfterVotingEndsInBlocks)]
    #[storage_mapper("governance:lockTimeAfterVotingEndsInBlocks")]
    fn lock_time_after_voting_ends_in_blocks(&self) -> SingleValueMapper<u64>;
}
