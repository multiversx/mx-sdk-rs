multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const MAX_GOVERNANCE_PROPOSAL_ACTIONS: usize = 4;
pub type ProposalId = usize;

pub type GovernanceActionAsMultiArg<M> =
    MultiValue4<u64, ManagedAddress<M>, ManagedBuffer<M>, ManagedVec<M, ManagedBuffer<M>>>;

#[derive(TypeAbi, TopEncode, TopDecode)]
pub enum VoteType {
    UpVote,
    DownVote,
    DownVetoVote,
    AbstainVote,
}

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Eq)]
pub enum GovernanceProposalStatus {
    None,
    Pending,
    Active,
    Defeated,
    Succeeded,
    Queued,
    WaitingForFees,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, TypeAbi)]
pub struct ProposalFees<M: ManagedTypeApi> {
    pub total_amount: BigUint<M>,
    pub entries: ManagedVec<M, FeeEntry<M>>,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, TypeAbi)]
pub struct FeeEntry<M: ManagedTypeApi> {
    pub depositor_addr: ManagedAddress<M>,
    pub tokens: EsdtTokenPayment<M>,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct GovernanceAction<M: ManagedTypeApi> {
    pub gas_limit: u64,
    pub dest_address: ManagedAddress<M>,
    pub function_name: ManagedBuffer<M>,
    pub arguments: ManagedVec<M, ManagedBuffer<M>>,
}

impl<M: ManagedTypeApi> GovernanceAction<M> {
    pub fn into_multiarg(self) -> GovernanceActionAsMultiArg<M> {
        (
            self.gas_limit,
            self.dest_address,
            self.function_name,
            self.arguments,
        )
            .into()
    }
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct GovernanceProposal<M: ManagedTypeApi> {
    pub proposer: ManagedAddress<M>,
    pub actions: ArrayVec<GovernanceAction<M>, MAX_GOVERNANCE_PROPOSAL_ACTIONS>,
    pub description: ManagedBuffer<M>,
    pub fees: ProposalFees<M>,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct ProposalVotes<M: ManagedTypeApi> {
    pub up_votes: BigUint<M>,
    pub down_votes: BigUint<M>,
    pub down_veto_votes: BigUint<M>,
    pub abstain_votes: BigUint<M>,
}

impl<M: ManagedTypeApi> Default for ProposalVotes<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: ManagedTypeApi> ProposalVotes<M> {
    pub fn new() -> Self {
        ProposalVotes {
            up_votes: BigUint::zero(),
            down_votes: BigUint::zero(),
            down_veto_votes: BigUint::zero(),
            abstain_votes: BigUint::zero(),
        }
    }

    pub fn get_total_votes(&self) -> BigUint<M> {
        &self.up_votes + &self.down_votes + &self.down_veto_votes + &self.abstain_votes
    }
    pub fn get_up_votes_percentage(&self) -> BigUint<M> {
        let total_votes = self.get_total_votes();
        &self.up_votes / &total_votes
    }
    pub fn get_down_votes_percentage(&self) -> BigUint<M> {
        let total_votes = self.get_total_votes();
        &self.down_votes / &total_votes
    }
    pub fn get_down_veto_votes_percentage(&self) -> BigUint<M> {
        let total_votes = self.get_total_votes();
        &self.down_veto_votes / &total_votes
    }
    pub fn get_abstain_votes_percentage(&self) -> BigUint<M> {
        let total_votes = self.get_total_votes();
        &self.abstain_votes / &total_votes
    }
}
