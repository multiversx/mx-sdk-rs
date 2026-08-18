use crate::{
    AbiType, AbiTypeFrom, HasUnmanaged, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName,
};

pub struct BigUintAbi;

// Cross-type AbiTypeFrom implementations
impl AbiTypeFrom<Self> for BigUintAbi {}
impl AbiTypeFrom<u8> for BigUintAbi {}
impl AbiTypeFrom<u16> for BigUintAbi {}
impl AbiTypeFrom<u32> for BigUintAbi {}
impl AbiTypeFrom<u64> for BigUintAbi {}
impl AbiTypeFrom<u128> for BigUintAbi {}

impl AbiType for BigUintAbi {
    fn type_name() -> TypeName {
        TypeName::from("BigUint")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbiFrom<Self> for BigUintAbi {}
impl TypeAbiFrom<u8> for BigUintAbi {}
impl TypeAbiFrom<u16> for BigUintAbi {}
impl TypeAbiFrom<u32> for BigUintAbi {}
impl TypeAbiFrom<u64> for BigUintAbi {}
impl TypeAbiFrom<u128> for BigUintAbi {}

impl TypeAbi for BigUintAbi {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        TypeName::from("BigUintAbi")
    }
}

#[cfg(feature = "num-bigint")]
impl HasUnmanaged for BigUintAbi {
    type Unmanaged = crate::codec::num_bigint::BigUint;
}

#[cfg(not(feature = "num-bigint"))]
impl HasUnmanaged for BigUintAbi {
    type Unmanaged = Self;
}
