use crate::{TypeAbi, TypeAbiFrom, TypeName};

/// Pure ABI counterpart of `BigFloat<M>`.
///
/// Provides a stable, framework-agnostic type representation for the floating-point big number type.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct BigFloatAbi;

impl TypeAbiFrom<Self> for BigFloatAbi {}

impl TypeAbi for BigFloatAbi {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        TypeName::from("BigFloat")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("BigFloatAbi")
    }
}
