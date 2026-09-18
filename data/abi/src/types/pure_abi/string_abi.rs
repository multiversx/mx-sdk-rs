use crate::{TypeAbi, TypeAbiFrom, TypeName};

pub struct StringAbi;

impl TypeAbiFrom<Self> for StringAbi {}

impl TypeAbi for StringAbi {
    type Abi = Self;

    fn type_name() -> TypeName {
        "utf-8 string".into()
    }

    fn type_name_rust() -> TypeName {
        "StringAbi".into()
    }
}
