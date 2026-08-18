use crate::{
    AbiType, AbiTypeFrom, HasUnmanaged, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName,
};

/// Pure ABI counterpart of `EllipticCurve<M>`.
///
/// Provides a stable, framework-agnostic type representation for the elliptic curve type.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct EllipticCurveAbi;

impl AbiTypeFrom<Self> for EllipticCurveAbi {}

impl AbiType for EllipticCurveAbi {
    fn type_name() -> TypeName {
        TypeName::from("EllipticCurve")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbiFrom<Self> for EllipticCurveAbi {}

impl TypeAbi for EllipticCurveAbi {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        TypeName::from("EllipticCurveAbi")
    }
}

impl HasUnmanaged for EllipticCurveAbi {
    type Unmanaged = Self;
}
