use elrond_wasm::api::{BigUintApi, EndpointFinishApi, ErrorApi, SendApi};
use elrond_wasm::io::EndpointResult;
use elrond_wasm::types::{Address, AsyncCall, BoxedBytes, CodeMetadata, SendEgld, Vec};
elrond_wasm::derive_imports!();

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
		egld_payment: BigUint,
		endpoint_name: BoxedBytes,
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

#[derive(TypeAbi)]
pub enum PerformActionResult<SA>
where
	SA: SendApi + 'static,
{
	Nothing,
	SendEgld(SendEgld<SA>),
	DeployResult(Address),
	AsyncCall(AsyncCall<SA>),
}

impl<FA, SA> EndpointResult<FA> for PerformActionResult<SA>
where
	FA: EndpointFinishApi + ErrorApi + Clone + 'static,
	SA: SendApi + Clone + 'static,
{
	fn finish(&self, api: FA) {
		match self {
			PerformActionResult::Nothing => (),
			PerformActionResult::SendEgld(send_egld) => send_egld.finish(api),
			PerformActionResult::DeployResult(address) => address.finish(api),
			PerformActionResult::AsyncCall(async_call) => async_call.finish(api),
		}
	}
}

#[cfg(test)]
mod test {
	use super::Action;
	use elrond_wasm_debug::api::RustBigUint;

	#[test]
	fn test_is_pending() {
		assert!(!Action::<RustBigUint>::Nothing.is_pending());
		assert!(Action::<RustBigUint>::ChangeQuorum(5).is_pending());
	}
}
