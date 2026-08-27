use crate::{AbiType, AbiTypeFrom, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName};

use super::BytesAbi;

pub struct StringAbi;

impl AbiTypeFrom<Self> for StringAbi {}
impl AbiTypeFrom<BytesAbi> for StringAbi {}
impl AbiTypeFrom<StringAbi> for BytesAbi {}

impl AbiType for StringAbi {
    fn type_name() -> TypeName {
        "utf-8 string".into()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbiFrom<Self> for StringAbi {}

impl TypeAbi for StringAbi {
    type Abi = Self;
}
