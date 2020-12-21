use elrond_wasm::{Address, BigUintApi, BoxedBytes, CodeMetadata, Vec};
derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum Action<BigUint: BigUintApi> {
	Nothing,
	AddBoardMember(Address),
	AddProposer(Address),
	RemoveUser(Address),
	ChangeQuorum(usize),
	SendEgld {
		to: Address,
		amount: BigUint,
		data: BoxedBytes,
	},
	SCDeploy {
		amount: BigUint,
		code: BoxedBytes,
		code_metadata: CodeMetadata,
		arguments: Vec<BoxedBytes>,
	},
	SCCall {
		to: Address,
		amount: BigUint,
		function: BoxedBytes,
		arguments: Vec<BoxedBytes>,
	},
}
