use crate::{
    abi::{
        ExplicitEnumVariantDescription, TypeAbi, TypeContents, TypeDescription,
        TypeDescriptionContainer, TypeName,
    },
    api::ManagedTypeApi,
    codec::{CodecFrom, EncodeErrorHandler, TopEncodeMulti, TopEncodeMultiOutput},
    types::ManagedBuffer,
};

const COMPLETED_STR: &str = "completed";
const INTERRUPTED_STR: &str = "interrupted";

/// Standard way of signalling that an operation was interrupted early, before running out of gas.
/// An endpoint that performs a longer operation can check from time to time if it is running low
/// on gas and can decide to save its state and exit, so that it can continue the same operation later.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum OperationCompletionStatus {
    Completed,
    InterruptedBeforeOutOfGas,
}

impl OperationCompletionStatus {
    pub fn output_bytes(&self) -> &'static [u8] {
        match self {
            OperationCompletionStatus::Completed => COMPLETED_STR.as_bytes(),
            OperationCompletionStatus::InterruptedBeforeOutOfGas => INTERRUPTED_STR.as_bytes(),
        }
    }

    pub fn is_completed(&self) -> bool {
        matches!(self, OperationCompletionStatus::Completed)
    }

    pub fn is_interrupted(&self) -> bool {
        matches!(self, OperationCompletionStatus::InterruptedBeforeOutOfGas)
    }
}

impl TopEncodeMulti for OperationCompletionStatus {
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        output.push_single_value(&self.output_bytes(), h)
    }
}

impl<M: ManagedTypeApi> CodecFrom<OperationCompletionStatus> for ManagedBuffer<M> {}
impl CodecFrom<OperationCompletionStatus> for crate::types::heap::BoxedBytes {}
impl CodecFrom<OperationCompletionStatus> for crate::types::heap::Vec<u8> {}

impl TypeAbi for OperationCompletionStatus {
    fn type_name() -> TypeName {
        TypeName::from("OperationCompletionStatus")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        let type_name = Self::type_name();

        accumulator.insert(
            type_name,
            TypeDescription {
                docs: &[],
                name: Self::type_name(),
                contents: TypeContents::ExplicitEnum([
                    ExplicitEnumVariantDescription {
                        docs: &["indicates that operation was completed"],
                        name: COMPLETED_STR,
                    },
                    ExplicitEnumVariantDescription {
                        docs: &["indicates that operation was interrupted prematurely, due to low gas"],
                        name: INTERRUPTED_STR,
                    }
                ].to_vec()),
            },
        );
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
