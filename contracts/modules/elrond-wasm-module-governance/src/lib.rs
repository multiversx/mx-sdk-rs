#![no_std]

elrond_wasm::imports!();

pub mod governance_configurable;

pub mod governance_proposal;
use governance_proposal::*;

#[elrond_wasm_derive::module]
pub trait GovernanceModule:
	governance_configurable::GovernanceConfigurablePropertiesModule
{
	// endpoints

	// views

	#[view(getProposalStatus)]
	fn get_proposal_status(&self, _proposal_id: usize) -> GovernanceProposalStatus {
		GovernanceProposalStatus::None
	}

	// private

	fn require_payment_token_governance_token(&self) -> SCResult<()> {
		require!(
			self.call_value().token() == self.governance_token_id().get(),
			"Only Governance token accepted as payment"
		);
		Ok(())
	}

	// storage - general

	#[storage_mapper("governance:proposals")]
	fn proposals(&self) -> VecMapper<Self::Storage, GovernanceProposal<Self::BigUint>>;

	#[view(getClaimableGovernanceTokens)]
	#[storage_mapper("governance:claimableGovernanceTokens")]
	fn claimable_governance_tokens(
		&self,
		address: &Address,
	) -> SingleValueMapper<Self::Storage, Self::BigUint>;
}
