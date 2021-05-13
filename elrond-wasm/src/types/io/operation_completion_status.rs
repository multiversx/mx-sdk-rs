use crate::{abi::TypeAbi, api::EndpointFinishApi, types::BoxedBytes, EndpointResult};
use alloc::string::String;

/// Standard way of signalling that an operation was interrupted early, before running out of gas.
/// An endpoint that performs a longer operation can check from time to time if it is running low
/// on gas and can decide to save its state and exit, so that it can continue the same operation later.
#[derive(PartialEq, Debug, Clone, Copy)]
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

	pub fn is_completed(&self) -> bool {
		matches!(self, OperationCompletionStatus::Completed)
	}

	pub fn is_interrupted(&self) -> bool {
		matches!(self, OperationCompletionStatus::InterruptedBeforeOutOfGas)
	}
}

impl EndpointResult for OperationCompletionStatus {
	type DecodeAs = BoxedBytes;

	#[inline]
	fn finish<FA>(&self, api: FA)
	where
		FA: EndpointFinishApi + Clone + 'static,
	{
		self.output_bytes().finish(api);
	}
}

impl TypeAbi for OperationCompletionStatus {
	fn type_name() -> String {
		String::from("OperationCompletionStatus")
	}
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_operation_completion_status_is() {
		assert!(OperationCompletionStatus::Completed.is_completed());
		assert!(!OperationCompletionStatus::Completed.is_interrupted());
		assert!(!OperationCompletionStatus::InterruptedBeforeOutOfGas.is_completed());
		assert!(OperationCompletionStatus::InterruptedBeforeOutOfGas.is_interrupted());
	}
}
