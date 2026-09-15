use crate::{TypeAbi, TypeAbiFrom, TypeName};

/// Pure ABI counterpart of `EllipticCurve<M>`.
///
/// Provides a stable, framework-agnostic type representation for the elliptic curve type.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct EllipticCurveAbi;

impl TypeAbiFrom<Self> for EllipticCurveAbi {}

impl TypeAbi for EllipticCurveAbi {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        TypeName::from("EllipticCurve")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("EllipticCurveAbi")
    }
}
