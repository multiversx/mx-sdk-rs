use std::fmt;

use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::ValueSubTree,
    value_interpreter::{interpret_string, interpret_subtree},
};

use super::AddressKey;

#[derive(PartialEq, Clone, Debug, Default)]
pub struct AddressValue {
    pub value: [u8; 32],
    pub original: ValueSubTree,
}

impl fmt::Display for AddressValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.original.fmt(f)
    }
}

pub(crate) fn value_from_slice(slice: &[u8]) -> [u8; 32] {
    let mut value = [0u8; 32];
    if slice.len() == 32 {
        value.copy_from_slice(slice);
    } else {
        panic!("account address is not 32 bytes in length");
    }
    value
}

impl InterpretableFrom<ValueSubTree> for AddressValue {
    fn interpret_from(from: ValueSubTree, context: &InterpreterContext) -> Self {
        let bytes = interpret_subtree(&from, context);
        AddressValue {
            value: value_from_slice(bytes.as_slice()),
            original: from,
        }
    }
}

impl InterpretableFrom<&str> for AddressValue {
    fn interpret_from(from: &str, context: &InterpreterContext) -> Self {
        let bytes = interpret_string(from, context);
        AddressValue {
            value: value_from_slice(bytes.as_slice()),
            original: ValueSubTree::Str(from.to_string()),
        }
    }
}

impl InterpretableFrom<&AddressKey> for AddressValue {
    fn interpret_from(from: &AddressKey, _context: &InterpreterContext) -> Self {
        AddressValue {
            value: from.value,
            original: ValueSubTree::Str(from.original.clone()),
        }
    }
}

impl InterpretableFrom<&[u8; 32]> for AddressValue {
    fn interpret_from(from: &[u8; 32], _context: &InterpreterContext) -> Self {
        AddressValue {
            value: from.clone(),
            original: ValueSubTree::Str(format!("0x{}", hex::encode(from))),
        }
    }
}

impl IntoRaw<ValueSubTree> for AddressValue {
    fn into_raw(self) -> ValueSubTree {
        self.original
    }
}
