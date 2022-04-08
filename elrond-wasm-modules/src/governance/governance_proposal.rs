use elrond_wasm::{
    api::ManagedTypeApi,
    arrayvec::ArrayVec,
    elrond_codec::multi_types::MultiValue7,
    types::{
        BigUint, ManagedAddress, ManagedBuffer, ManagedVec, MultiValueEncoded, TokenIdentifier,
    },
};

elrond_wasm::derive_imports!();

pub type GovernanceActionAsMultiArg<M> = MultiValue7<
    u64,
    ManagedAddress<M>,
    TokenIdentifier<M>,
    u64,
    BigUint<M>,
    ManagedBuffer<M>,
    ManagedVec<M, ManagedBuffer<M>>,
>;

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq)]
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
    pub token_id: TokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
    pub function_name: ManagedBuffer<M>,
    pub arguments: ManagedVec<M, ManagedBuffer<M>>,
}

impl<M: ManagedTypeApi> GovernanceAction<M> {
    pub fn into_multiarg(self) -> GovernanceActionAsMultiArg<M> {
        (
            self.gas_limit,
            self.dest_address,
            self.token_id,
            self.token_nonce,
            self.amount,
            self.function_name,
            self.arguments,
        )
            .into()
    }
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct GovernanceProposal<M: ManagedTypeApi, CAP: usize> {
    pub proposer: ManagedAddress<M>,
    pub actions: ArrayVec<GovernanceAction<M>, CAP>,
    pub description: ManagedBuffer<M>,
}
