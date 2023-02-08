use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::NewAddressRaw,
};

use super::{AddressValue, U64Value};

#[derive(Debug, Clone)]
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

impl IntoRaw<NewAddressRaw> for NewAddress {
    fn into_raw(self) -> NewAddressRaw {
        NewAddressRaw {
            creator_address: self.creator_address.original,
            creator_nonce: self.creator_nonce.original,
            new_address: self.new_address.original,
        }
    }
}
