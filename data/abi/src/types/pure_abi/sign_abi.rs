use crate::{TypeAbi, TypeAbiFrom, TypeName};

/// Pure ABI counterpart of `Sign` (the sign of a `BigInt`).
///
/// Provides a stable, framework-agnostic type representation for the sign enum.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct SignAbi;

impl TypeAbiFrom<Self> for SignAbi {}

impl TypeAbi for SignAbi {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        TypeName::from("Sign")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("SignAbi")
    }
}
