use crate::{
	abi::TypeAbi,
	api::{EndpointFinishApi, ErrorApi},
	EndpointResult,
};
use alloc::string::String;

/// Standard way of signalling that an operation was interrupted early, before running out of gas.
/// An endpoint that performs a longer operation can check from time to time if it is running low
/// on gas and can decide to save its state and exit, so that it can continue the same operation later.
pub enum OperationCompletionStatus {
	Completed,
	InterruptedBeforeOutOfGas,
}

impl OperationCompletionStatus {
	pub fn output_bytes(&self) -> &'static [u8] {
		match self {
			OperationCompletionStatus::Completed => b"completed",
			OperationCompletionStatus::InterruptedBeforeOutOfGas => b"interrupted",
		}
	}
}

impl<FA> EndpointResult<FA> for OperationCompletionStatus
where
	FA: EndpointFinishApi + ErrorApi + Clone + 'static,
{
	#[inline]
	fn finish(&self, api: FA) {
		self.output_bytes().finish(api);
	}
}

impl TypeAbi for OperationCompletionStatus {
	fn type_name() -> String {
		String::from("OperationCompletionStatus")
	}
}
