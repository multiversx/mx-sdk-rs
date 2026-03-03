use crate::codec::num_bigint::{BigInt, BigUint};

use super::{TypeAbi, TypeAbiFrom, TypeName};

impl TypeAbiFrom<Self> for BigUint {}
impl TypeAbiFrom<&Self> for BigUint {}

impl TypeAbi for BigUint {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        TypeName::from("BigUint")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("num_bigint::BigUint")
    }
}

impl TypeAbiFrom<Self> for BigInt {}
impl TypeAbiFrom<&Self> for BigInt {}

impl TypeAbi for BigInt {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        TypeName::from("BigInt")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("num_bigint::BigInt")
    }
}
