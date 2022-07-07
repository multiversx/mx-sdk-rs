elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub const MAX_GOVERNANCE_PROPOSAL_ACTIONS: usize = 5;

pub type GovernanceActionAsMultiArg<M> = MultiValue5<
    u64,
    ManagedAddress<M>,
    ManagedVec<M, EsdtTokenPayment<M>>,
    ManagedBuffer<M>,
    ManagedVec<M, ManagedBuffer<M>>,
>;

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Eq)]
pub enum GovernanceProposalStatus {
    None,
    Pending,
    Active,
    Defeated,
    Succeeded,
    Queued,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct GovernanceAction<M: ManagedTypeApi> {
    pub gas_limit: u64,
    pub dest_address: ManagedAddress<M>,
    pub payments: ManagedVec<M, EsdtTokenPayment<M>>,
    pub function_name: ManagedBuffer<M>,
    pub arguments: ManagedVec<M, ManagedBuffer<M>>,
}

impl<M: ManagedTypeApi> GovernanceAction<M> {
    pub fn into_multiarg(self) -> GovernanceActionAsMultiArg<M> {
        (
            self.gas_limit,
            self.dest_address,
            self.payments,
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
}
