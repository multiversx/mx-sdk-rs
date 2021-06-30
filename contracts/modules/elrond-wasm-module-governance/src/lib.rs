#![no_std]

elrond_wasm::imports!();

#[elrond_wasm_derive::module]
pub trait GovernanceModule {
	// endpoint - owner-only

	/// Will do nothing on a second call, or if the storage is modified manually beforehand from the main SC
	#[endpoint(initGovernanceModule)]
	fn init_governance_module(
		&self,
		governance_token_id: TokenIdentifier,
		timelock_sc_address: Address,
		quorum: Self::BigUint,
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

		self.governance_token_id()
			.set_if_empty(&governance_token_id);
		self.timelock_sc_address()
			.set_if_empty(&timelock_sc_address);
		self.quorum().set_if_empty(&quorum);

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

	// private

	fn require_caller_timelock_sc(&self) -> SCResult<()> {
		require!(
			self.blockchain().get_caller() == self.timelock_sc_address().get(),
			"Only timelock SC may call this endpoint."
		);
		Ok(())
	}

	// storage

	#[view(getGovernanceTokenId)]
	#[storage_mapper("governance:governanceTokenId")]
	fn governance_token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

	#[view(getClaimableGovernanceTokens)]
	#[storage_mapper("governance:claimableGovernanceTokens")]
	fn claimable_governance_tokens(
		&self,
		address: &Address,
	) -> SingleValueMapper<Self::Storage, Self::BigUint>;

	#[view(getTimelockScAddress)]
	#[storage_mapper("governance:timelockScAddress")]
	fn timelock_sc_address(&self) -> SingleValueMapper<Self::Storage, Address>;

	#[view(getQuorum)]
	#[storage_mapper("governance:quorum")]
	fn quorum(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;
}
