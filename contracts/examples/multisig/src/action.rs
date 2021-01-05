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

impl<BigUint: BigUintApi> Action<BigUint> {
	/// Only pending actions are kept in storage,
	/// both executed and discarded actions are removed (converted to `Nothing`).
	/// So this is equivalent to `action != Action::Nothing`.
	pub fn is_pending(&self) -> bool {
		!matches!(*self, Action::Nothing)
	}
}

/// Not used internally, just to retrieve results via endpoint. 
#[derive(TopEncode, TypeAbi)]
pub struct ActionFullInfo<BigUint: BigUintApi> {
	pub action_id: usize,
	pub action_data: Action<BigUint>,
	pub signers: Vec<Address>,
}

#[cfg(test)]
mod test {
	use super::Action;
	use elrond_wasm_debug::RustBigUint;

	#[test]
	fn test_is_pending() {
		assert!(!Action::<RustBigUint>::Nothing.is_pending());
		assert!(Action::<RustBigUint>::ChangeQuorum(5).is_pending());
	}
}
