use elrond_wasm::{
	api::BigUintApi,
	types::{Address, BoxedBytes, MultiArg7, TokenIdentifier},
	Vec,
};

elrond_wasm::derive_imports!();

pub type GovernanceActionAsMultiArg<BigUint> =
	MultiArg7<u64, Address, TokenIdentifier, u64, BigUint, BoxedBytes, Vec<BoxedBytes>>;

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
	pub gas_limit: u64,
	pub dest_address: Address,
	pub token_id: TokenIdentifier,
	pub token_nonce: u64,
	pub amount: BigUint,
	pub function_name: BoxedBytes,
	pub arguments: Vec<BoxedBytes>,
}

impl<BigUint: BigUintApi> GovernanceAction<BigUint> {
	pub fn into_multiarg(self) -> GovernanceActionAsMultiArg<BigUint> {
		(
			self.gas_limit,
			self.dest_address,
			self.token_id,
			self.token_nonce,
			self.amount,
			self.function_name,
			self.arguments.into(),
		)
			.into()
	}
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct GovernanceProposal<BigUint: BigUintApi> {
	pub proposer: Address,
	pub actions: Vec<GovernanceAction<BigUint>>,
	pub description: BoxedBytes,
}
