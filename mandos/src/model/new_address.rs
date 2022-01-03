use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::NewAddressRaw,
};

use super::{AddressValue, U64Value};

#[derive(Debug)]
pub struct NewAddress {
    pub creator_address: AddressValue,
    pub creator_nonce: U64Value,
    pub new_address: AddressValue,
}

impl InterpretableFrom<NewAddressRaw> for NewAddress {
    fn interpret_from(from: NewAddressRaw, context: &InterpreterContext) -> Self {
        NewAddress {
            creator_address: AddressValue::interpret_from(from.creator_address, context),
            creator_nonce: U64Value::interpret_from(from.creator_nonce, context),
            new_address: AddressValue::interpret_from(from.new_address, context),
        }
    }
}
