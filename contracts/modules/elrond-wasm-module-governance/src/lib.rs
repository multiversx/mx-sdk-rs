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

	// Used to deposit tokens for "payable" actions
	// There is no "withdraw" functionality
	// Funds can only be retrived through an action
	#[payable("*")]
	#[endpoint(depositTokensForAction)]
	fn deposit_tokens_for_action(&self) {}

	#[payable("*")]
	#[endpoint]
	fn propose(
		&self,
		#[payment_amount] payment_amount: Self::BigUint,
		description: BoxedBytes,
		#[var_args] actions: VarArgs<GovernanceActionAsMultiArg<Self::BigUint>>,
	) -> SCResult<usize> {
		self.require_payment_token_governance_token()?;
		require!(
			payment_amount >= self.min_token_balance_for_proposing().get(),
			"Not enough tokens for proposing action"
		);
		require!(
			actions.len() <= self.max_actions_per_proposal().get(),
			"Exceeded max actions per proposal"
		);

		let mut gov_actions = Vec::with_capacity(actions.len());
		for action in actions.into_vec() {
			let (dest_address, token_id, amount, function_name, arguments) = action.into_tuple();
			let gov_action = GovernanceAction {
				dest_address,
				token_id,
				amount,
				function_name,
				arguments,
			};

			gov_actions.push(gov_action);
		}

		let proposer = self.blockchain().get_caller();
		let proposal = GovernanceProposal {
			proposer: proposer.clone(),
			description,
			actions: gov_actions,
		};
		let proposal_id = self.proposals().push(&proposal);

		self.total_votes(proposal_id).set(&payment_amount);
		self.votes(proposal_id).insert(proposer, payment_amount);

		Ok(proposal_id)
	}

	// views

	#[view(getProposalStatus)]
	fn get_proposal_status(&self, proposal_id: usize) -> GovernanceProposalStatus {
		if !self.is_valid_proposal_id(proposal_id) {
			return GovernanceProposalStatus::None;
		}

		GovernanceProposalStatus::Canceled
	}

	// private

	fn require_payment_token_governance_token(&self) -> SCResult<()> {
		require!(
			self.call_value().token() == self.governance_token_id().get(),
			"Only Governance token accepted as payment"
		);
		Ok(())
	}

	fn is_valid_proposal_id(&self, proposal_id: usize) -> bool {
		proposal_id >= 1 && proposal_id <= self.proposals().len()
	}

	fn get_sc_balance(&self, token_id: &TokenIdentifier) -> Self::BigUint {
		if token_id.is_egld() {
			self.blockchain().get_sc_balance()
		} else {
			self.blockchain()
				.get_esdt_balance(&self.blockchain().get_sc_address(), token_id, 0)
		}
	}

	// storage - general

	#[storage_mapper("governance:proposals")]
	fn proposals(&self) -> VecMapper<Self::Storage, GovernanceProposal<Self::BigUint>>;

	#[storage_mapper("governance:votes")]
	fn votes(&self, proposal_id: usize) -> MapMapper<Self::Storage, Address, Self::BigUint>;

	#[storage_mapper("governance:downvotes")]
	fn downvotes(&self, proposal_id: usize) -> MapMapper<Self::Storage, Address, Self::BigUint>;

	/// Could be calculated by iterating over the "votes" mapper, but that costs a lot of gas
	#[view(getTotalVotes)]
	#[storage_mapper("governance:totalVotes")]
	fn total_votes(&self, proposal_id: usize) -> SingleValueMapper<Self::Storage, Self::BigUint>;

	/// Could be calculated by iterating over the "downvotes" mapper, but that costs a lot of gas
	#[view(getTotalDownvotes)]
	#[storage_mapper("governance:totalDownvotes")]
	fn total_downvotes(
		&self,
		proposal_id: usize,
	) -> SingleValueMapper<Self::Storage, Self::BigUint>;
}
