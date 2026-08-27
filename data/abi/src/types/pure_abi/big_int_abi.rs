use crate::{AbiType, AbiTypeFrom, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName};

/// Pure ABI counterpart of `BigInt<M>`.
///
/// Provides a stable, framework-agnostic type representation for the signed big integer type.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct BigIntAbi;

impl AbiTypeFrom<Self> for BigIntAbi {}
impl AbiTypeFrom<i8> for BigIntAbi {}
impl AbiTypeFrom<i16> for BigIntAbi {}
impl AbiTypeFrom<i32> for BigIntAbi {}
impl AbiTypeFrom<i64> for BigIntAbi {}

impl AbiType for BigIntAbi {
    fn type_name() -> TypeName {
        TypeName::from("BigInt")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbiFrom<Self> for BigIntAbi {}

impl TypeAbi for BigIntAbi {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        TypeName::from("BigIntAbi")
    }
}
