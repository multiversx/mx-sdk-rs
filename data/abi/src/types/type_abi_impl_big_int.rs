use crate::codec::num_bigint::{BigInt, BigUint};

use super::{BigIntAbi, BigUintAbi, TypeAbi, TypeAbiFrom, TypeName};

impl TypeAbiFrom<Self> for BigUint {}
impl TypeAbiFrom<&Self> for BigUint {}
impl TypeAbiFrom<BigUintAbi> for BigUint {}

impl TypeAbi for BigUint {
    type Abi = BigUintAbi;

    fn type_name_rust() -> TypeName {
        TypeName::from("num_bigint::BigUint")
    }
}

impl TypeAbiFrom<Self> for BigInt {}
impl TypeAbiFrom<&Self> for BigInt {}
impl TypeAbiFrom<BigIntAbi> for BigInt {}

impl TypeAbi for BigInt {
    type Abi = BigIntAbi;

    fn type_name_rust() -> TypeName {
        TypeName::from("num_bigint::BigInt")
    }
}
