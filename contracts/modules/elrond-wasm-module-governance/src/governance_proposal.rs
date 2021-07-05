use elrond_wasm::{
	api::BigUintApi,
	types::{Address, BoxedBytes, MultiArg5, TokenIdentifier},
	Vec,
};

elrond_wasm::derive_imports!();

pub type GovernanceActionAsMultiArg<BigUint> =
	MultiArg5<Address, TokenIdentifier, BigUint, BoxedBytes, Vec<BoxedBytes>>;

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
pub struct GovernanceAction<BigUint: BigUintApi> {
	pub dest_address: Address,
	pub token_id: TokenIdentifier,
	pub amount: BigUint,
	pub function_name: BoxedBytes,
	pub arguments: Vec<BoxedBytes>,
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct GovernanceProposal<BigUint: BigUintApi> {
	pub proposer: Address,
	pub actions: Vec<GovernanceAction<BigUint>>,
	pub description: BoxedBytes,
}
