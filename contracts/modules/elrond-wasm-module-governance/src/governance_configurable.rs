elrond_wasm::imports!();

#[elrond_wasm_derive::module]
pub trait GovernanceConfigurablePropertiesModule {
	// endpoints - owner-only

	/// Will do nothing on a second call, or if the storage is modified manually beforehand from the main SC
	/// The module can't protect its storage from the main SC, so it's the developers responsibility
	/// to not modify parameters recklessly
	#[endpoint(initGovernanceModule)]
	fn init_governance_module(
		&self,
		governance_token_id: TokenIdentifier,
		timelock_sc_address: Address,
		quorum: Self::BigUint,
		min_token_balance_for_proposal: Self::BigUint,
		max_actions_per_proposal: usize,
		voting_delay_in_blocks: u64,
		voting_period_in_blocks: u64,
	) -> SCResult<()> {
		only_owner!(self, "Only owner may initialize governance module");
		require!(
			governance_token_id.is_valid_esdt_identifier(),
			"Invalid ESDT token ID provided for governance_token_id"
		);
		require!(
			self.blockchain().is_smart_contract(&timelock_sc_address),
			"Timelock address provided is not a SC address"
		);
		require!(quorum > 0, "Quorum can't be set to 0");
		require!(
			min_token_balance_for_proposal > 0,
			"Min token balance for proposing can't be set to 0"
		);
		require!(
			max_actions_per_proposal > 0,
			"Max actions per proposal can't be set to 0"
		);
		require!(
			voting_period_in_blocks > 0,
			"Voting period (in blocks) can't be set to 0"
		);

		self.governance_token_id()
			.set_if_empty(&governance_token_id);

		self.timelock_sc_address()
			.set_if_empty(&timelock_sc_address);

		self.quorum().set_if_empty(&quorum);

		self.min_token_balance_for_proposing()
			.set_if_empty(&min_token_balance_for_proposal);

		self.max_actions_per_proposal()
			.set_if_empty(&max_actions_per_proposal);

		self.voting_delay_in_blocks()
			.set_if_empty(&voting_delay_in_blocks);

		self.voting_period_in_blocks()
			.set_if_empty(&voting_period_in_blocks);

		Ok(())
	}

	// endpoints - timelock SC-only

	#[endpoint(changeQuorum)]
	fn change_quorum(&self, new_quorum: Self::BigUint) -> SCResult<()> {
		self.require_caller_timelock_sc()?;
		require!(new_quorum > 0, "Quorum can't be set to 0");

		self.quorum().set(&new_quorum);

		Ok(())
	}

	#[endpoint(changeMinTokenBalanceForProposing)]
	fn change_min_token_balance_for_proposing(&self, new_value: Self::BigUint) -> SCResult<()> {
		self.require_caller_timelock_sc()?;
		require!(
			new_value > 0,
			"Min token balance for proposing can't be set to 0"
		);

		self.min_token_balance_for_proposing().set(&new_value);

		Ok(())
	}

	#[endpoint(changeMaxActionsPerProposal)]
	fn change_max_actions_per_proposal(&self, new_value: usize) -> SCResult<()> {
		self.require_caller_timelock_sc()?;
		require!(new_value > 0, "Max actions per proposal can't be set to 0");

		self.max_actions_per_proposal().set(&new_value);

		Ok(())
	}

	#[endpoint(changeVotingDelayInBlocks)]
	fn change_voting_delay_in_blocks(&self, new_value: u64) -> SCResult<()> {
		self.require_caller_timelock_sc()?;

		self.voting_delay_in_blocks().set(&new_value);

		Ok(())
	}

	#[endpoint(changeVotingPeriodInBlocks)]
	fn change_voting_period_in_blocks(&self, new_value: u64) -> SCResult<()> {
		self.require_caller_timelock_sc()?;
		require!(new_value > 0, "Voting period (in blocks) can't be set to 0");

		self.voting_period_in_blocks().set(&new_value);

		Ok(())
	}

	// private

	fn require_caller_timelock_sc(&self) -> SCResult<()> {
		require!(
			self.blockchain().get_caller() == self.timelock_sc_address().get(),
			"Only timelock SC may call this endpoint"
		);
		Ok(())
	}

	// storage - fixed parameters

	#[view(getGovernanceTokenId)]
	#[storage_mapper("governance:governanceTokenId")]
	fn governance_token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

	#[view(getTimelockScAddress)]
	#[storage_mapper("governance:timelockScAddress")]
	fn timelock_sc_address(&self) -> SingleValueMapper<Self::Storage, Address>;

	// storage - configurable parameters

	#[view(getQuorum)]
	#[storage_mapper("governance:quorum")]
	fn quorum(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

	#[view(getMinTokenBalanceForProposing)]
	#[storage_mapper("governance:minTokenBalanceForProposing")]
	fn min_token_balance_for_proposing(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

	#[view(getMaxActionsPerProposal)]
	#[storage_mapper("governance:maxActionsPerProposal")]
	fn max_actions_per_proposal(&self) -> SingleValueMapper<Self::Storage, usize>;

	#[view(getVotingDelayInBlocks)]
	#[storage_mapper("governance:votingDelayInBlocks")]
	fn voting_delay_in_blocks(&self) -> SingleValueMapper<Self::Storage, u64>;

	#[view(getVotingPeriodInBlocks)]
	#[storage_mapper("governance:votingPeriodInBlocks")]
	fn voting_period_in_blocks(&self) -> SingleValueMapper<Self::Storage, u64>;
}
