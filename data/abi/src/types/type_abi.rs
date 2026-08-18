use super::*;
use crate::contract_abi::{OutputAbi, OutputAbis};
use alloc::{format, string::ToString, vec::Vec};

/// Abstract ABI type descriptor.
///
/// Owned by the ABI name, JSON schema generation, and type descriptions.
/// Implemented by all pure ABI types (e.g. `BigUintAbi`, `AddressAbi`),
/// primitives (u32, bool, etc.), and framework-agnostic types (H256, etc.).
///
/// NOT implemented by managed types with type parameters (e.g. `BigInt<M>`).
pub trait AbiType {
    /// The type name, as it shows up in the ABI.
    fn type_name() -> TypeName;

    /// Specific name to be optionally added to the ABI.
    ///
    /// Added to allow adding more type information to the ABI, in a backwards compatible manner.
    /// This is important, since we currently do not encode the original Rust type information.
    fn type_name_specific() -> Option<TypeName> {
        None
    }

    /// A type can provide more than its own name.
    /// For instance, a struct can also provide the descriptions of the type of its fields.
    /// AbiType doesn't care for the exact accumulator type,
    /// which is abstracted by the TypeDescriptionContainer trait.
    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC);

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
    fn output_abis(output_names: &[&'static str]) -> OutputAbis {
        let mut result = Vec::with_capacity(1);
        let output_name = if !output_names.is_empty() {
            output_names[0]
        } else {
            ""
        };
        result.push(OutputAbi {
            output_name: output_name.to_string(),
            type_names: TypeNames {
                abi: Self::type_name(),
                rust: Self::type_name(),
                specific: Self::type_name_specific(),
            },
            multi_result: Self::is_variadic(),
        });
        result
    }
}

/// Implemented for all concrete types that can end up in the ABI:
/// - argument types,
/// - result types,
/// - event log arguments
/// - etc.
///
/// Will be automatically implemented for struct and enum types via the `#[type_abi]` annotation.
pub trait TypeAbi: TypeAbiFrom<Self> {
    /// The pure ABI type, without any managed API type parameters.
    /// For most types this is `Self`, but for managed types (e.g. `BigUint`) it points to a dedicated ABI counterpart.
    type Abi: AbiType;

    /// The type name as it appears in the ABI.
    ///
    /// Kept for backwards compatibility. New code should use `Self::Abi::type_name()`.
    fn type_name() -> TypeName {
        Self::Abi::type_name()
    }

    /// The type name as it shows up in Rust code. Used for proxies.
    ///
    /// Does not get saved into the ABI, but is used for code generation.
    fn type_name_rust() -> TypeName {
        core::any::type_name::<Self>().into()
    }

    fn type_names() -> TypeNames {
        TypeNames {
            abi: Self::Abi::type_name(),
            rust: Self::type_name_rust(),
            specific: Self::Abi::type_name_specific(),
        }
    }

    /// A type can provide more than its own name.
    /// For instance, a struct can also provide the descriptions of the type of its fields.
    /// TypeAbi delegates to its Abi type.
    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        Self::Abi::provide_type_descriptions(accumulator);
    }

    #[doc(hidden)]
    fn is_variadic() -> bool {
        Self::Abi::is_variadic()
    }

    /// Method that provides output ABIs directly.
    /// All types should return a single output, since Rust only allows for single method results
    /// (even if it is a multi-output, live MultiResultVec),
    /// however, MultiResultX when top-level can be seen as multiple endpoint results.
    /// This method gives it an opportunity to dissolve into its components.
    /// Should only be overridden by framework types.
    /// Output names are optionally provided in contracts via the `output_name` method attribute.
    #[doc(hidden)]
    fn output_abis(output_names: &[&'static str]) -> OutputAbis {
        Self::Abi::output_abis(output_names)
    }
}

pub fn type_name_variadic<T: AbiType>() -> TypeName {
    format!("variadic<{}>", T::type_name())
}

pub fn type_name_multi_value_encoded<T: TypeAbi>() -> TypeName {
    format!("MultiValueEncoded<$API, {}>", T::type_name_rust())
}

pub fn type_name_optional<T: AbiType>() -> TypeName {
    let mut repr = TypeName::from("optional<");
    repr.push_str(T::type_name().as_str());
    repr.push('>');
    repr
}
