use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    value_interpreter::interpret_string,
};

use std::{cmp::Ordering, fmt};

use super::{value_from_slice, AddressValue};

#[derive(Debug, Clone, Eq, Default)]
pub struct AddressKey {
    pub value: [u8; 32],
    pub original: String,
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
            value: from.value,
            original: from.original.to_string(),
        }
    }
}
