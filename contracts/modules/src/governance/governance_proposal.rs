multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const MAX_GOVERNANCE_PROPOSAL_ACTIONS: usize = 4;
pub type ProposalId = usize;

pub type GovernanceActionAsMultiArg<'a, M> =
    MultiValue4<u64, ManagedAddress<'a, M>, ManagedBuffer<'a, M>, ManagedVec<'a, M, ManagedBuffer<'a, M>>>;

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
pub struct ProposalFees<'a, M: ManagedTypeApi<'a>> {
    pub total_amount: BigUint<'a, M>,
    pub entries: ManagedVec<'a, M, FeeEntry<'a, M>>,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, TypeAbi)]
pub struct FeeEntry<'a, M: ManagedTypeApi<'a>> {
    pub depositor_addr: ManagedAddress<'a, M>,
    pub tokens: EsdtTokenPayment<'a, M>,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct GovernanceAction<'a, M: ManagedTypeApi<'a>> {
    pub gas_limit: u64,
    pub dest_address: ManagedAddress<'a, M>,
    pub function_name: ManagedBuffer<'a, M>,
    pub arguments: ManagedVec<'a, M, ManagedBuffer<'a, M>>,
}

impl<'a, M: ManagedTypeApi<'a>> GovernanceAction<'a, M> {
    pub fn into_multiarg(self) -> GovernanceActionAsMultiArg<'a, M> {
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
pub struct GovernanceProposal<'a, M: ManagedTypeApi<'a>> {
    pub proposer: ManagedAddress<'a, M>,
    pub actions: ArrayVec<GovernanceAction<'a, M>, MAX_GOVERNANCE_PROPOSAL_ACTIONS>,
    pub description: ManagedBuffer<'a, M>,
    pub fees: ProposalFees<'a, M>,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct ProposalVotes<'a, M: ManagedTypeApi<'a>> {
    pub up_votes: BigUint<'a, M>,
    pub down_votes: BigUint<'a, M>,
    pub down_veto_votes: BigUint<'a, M>,
    pub abstain_votes: BigUint<'a, M>,
}

impl<'a, M: ManagedTypeApi<'a>> Default for ProposalVotes<'a, M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, M: ManagedTypeApi<'a>> ProposalVotes<'a, M> {
    pub fn new() -> Self {
        ProposalVotes {
            up_votes: BigUint::zero(),
            down_votes: BigUint::zero(),
            down_veto_votes: BigUint::zero(),
            abstain_votes: BigUint::zero(),
        }
    }

    pub fn get_total_votes(&self) -> BigUint<'a, M> {
        &self.up_votes + &self.down_votes + &self.down_veto_votes + &self.abstain_votes
    }
    pub fn get_up_votes_percentage(&self) -> BigUint<'a, M> {
        let total_votes = self.get_total_votes();
        &self.up_votes / &total_votes
    }
    pub fn get_down_votes_percentage(&self) -> BigUint<'a, M> {
        let total_votes = self.get_total_votes();
        &self.down_votes / &total_votes
    }
    pub fn get_down_veto_votes_percentage(&self) -> BigUint<'a, M> {
        let total_votes = self.get_total_votes();
        &self.down_veto_votes / &total_votes
    }
    pub fn get_abstain_votes_percentage(&self) -> BigUint<'a, M> {
        let total_votes = self.get_total_votes();
        &self.abstain_votes / &total_votes
    }
}
