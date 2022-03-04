use super::*;
use alloc::{string::String, vec::Vec};

pub trait TypeAbi {
    fn type_name() -> String {
        core::any::type_name::<Self>().into()
    }

    /// A type can provide more than its own description.
    /// For instance, a struct can also provide the descriptions of the type of its fields.
    /// TypeAbi doesn't care for the exact accumulator type,
    /// which is abstracted by the TypeDescriptionContainer trait.
    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        let type_name = Self::type_name();
        accumulator.insert(
            type_name,
            TypeDescription {
                docs: &[],
                name: Self::type_name(),
                contents: TypeContents::NotSpecified,
            },
        );
    }

    #[doc(hidden)]
    fn is_variadic() -> bool {
        false
    }

    /// Method that provides output ABIs directly.
    /// All types should return a single output, since Rust only allows for single method results
    /// (even if it is a multi-output, live MultiResultVec),
    /// however, MultiResultX when top-level can be seen as multiple endpoint results.
    /// This method gives it an opportunity to dissolve into its components.
    /// Should only be overridden by framework types.
    /// Output names are optionally provided in contracts via the `output_name` method attribute.
    #[doc(hidden)]
    fn output_abis(output_names: &[&'static str]) -> Vec<OutputAbi> {
        let mut result = Vec::with_capacity(1);
        let output_name = if !output_names.is_empty() {
            output_names[0]
        } else {
            ""
        };
        result.push(OutputAbi {
            output_name,
            type_name: Self::type_name(),
            multi_result: Self::is_variadic(),
        });
        result
    }
}
