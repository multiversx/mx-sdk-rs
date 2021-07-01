use elrond_wasm::{
	api::BigUintApi,
	types::{Address, BoxedBytes},
	Vec,
};

elrond_wasm::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq)]
pub enum GovernanceProposalStatus {
	None,
	Pending,
	Active,
	Defeated,
	Succeeded,
	Queued,
	Expired,
	Executed,
	Canceled,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct GovernanceAction<BigUint: BigUintApi> {
	pub dest_address: Address,
	pub call_value: BigUint,
	pub function_name: BoxedBytes,
	pub arguments: Vec<BoxedBytes>,
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct GovernanceProposal<BigUint: BigUintApi> {
	pub proposer: Address,
	pub actions: Vec<GovernanceAction<BigUint>>,
	pub description: BoxedBytes,
}
