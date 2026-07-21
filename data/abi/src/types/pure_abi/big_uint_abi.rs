use crate::{TypeAbi, TypeAbiFrom, TypeName};

pub struct BigUintAbi;

impl TypeAbiFrom<Self> for BigUintAbi {}

impl TypeAbi for BigUintAbi {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        TypeName::from("BigUint")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("BigUintAbi")
    }
}
