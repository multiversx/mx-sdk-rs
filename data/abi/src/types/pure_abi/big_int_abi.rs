use crate::{TypeAbi, TypeAbiFrom, TypeName};

/// Pure ABI counterpart of `BigInt<M>`.
///
/// Provides a stable, framework-agnostic type representation for the signed big integer type.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct BigIntAbi;

impl TypeAbiFrom<Self> for BigIntAbi {}

impl TypeAbi for BigIntAbi {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        TypeName::from("BigInt")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("BigIntAbi")
    }
}
