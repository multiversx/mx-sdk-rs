use crate::{TypeAbi, TypeAbiFrom, TypeName};

pub struct BigUintAbi;

impl TypeAbiFrom<Self> for BigUintAbi {}

impl TypeAbiFrom<u8> for BigUintAbi {}
impl TypeAbiFrom<u16> for BigUintAbi {}
impl TypeAbiFrom<u32> for BigUintAbi {}
impl TypeAbiFrom<u64> for BigUintAbi {}
impl TypeAbiFrom<u128> for BigUintAbi {}

impl TypeAbi for BigUintAbi {
    type Abi = Self;

    fn type_name() -> TypeName {
        TypeName::from("BigUint")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("BigUintAbi")
    }
}
