use super::{value_from_slice, AddressValue};
use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    value_interpreter::interpret_string,
};
use elrond_wasm::types::Address;
use std::{cmp::Ordering, fmt};

#[derive(Debug, Clone, Eq)]
pub struct AddressKey {
    pub value: Address,
    pub original: String,
}

impl Default for AddressKey {
    fn default() -> Self {
        Self {
            value: Address::zero(),
            original: Default::default(),
        }
    }
}

impl AddressKey {
    pub fn to_address(&self) -> Address {
        self.value.clone()
    }
}

impl Ord for AddressKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.original.cmp(&other.original)
    }
}

impl PartialOrd for AddressKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for AddressKey {
    fn eq(&self, other: &Self) -> bool {
        self.original == other.original
    }
}

impl fmt::Display for AddressKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.original.fmt(f)
    }
}

impl InterpretableFrom<&str> for AddressKey {
    fn interpret_from(from: &str, context: &InterpreterContext) -> Self {
        let bytes = interpret_string(from, context);
        AddressKey {
            value: value_from_slice(bytes.as_slice()),
            original: from.to_string(),
        }
    }
}

impl InterpretableFrom<String> for AddressKey {
    fn interpret_from(from: String, context: &InterpreterContext) -> Self {
        AddressKey::interpret_from(from.as_str(), context)
    }
}

impl InterpretableFrom<&AddressValue> for AddressKey {
    fn interpret_from(from: &AddressValue, _context: &InterpreterContext) -> Self {
        AddressKey {
            value: from.to_address(),
            original: from.original.to_concatenated_string(),
        }
    }
}

impl InterpretableFrom<AddressValue> for AddressKey {
    fn interpret_from(from: AddressValue, context: &InterpreterContext) -> Self {
        AddressKey::interpret_from(&from, context)
    }
}

impl InterpretableFrom<&Address> for AddressKey {
    fn interpret_from(from: &Address, _context: &InterpreterContext) -> Self {
        AddressKey {
            value: from.clone(),
            original: format!("0x{}", hex::encode(from)),
        }
    }
}

impl IntoRaw<String> for AddressKey {
    fn into_raw(self) -> String {
        self.original
    }
}
