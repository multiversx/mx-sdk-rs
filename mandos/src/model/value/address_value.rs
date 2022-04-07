use std::fmt;

use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::ValueSubTree,
    value_interpreter::{interpret_string, interpret_subtree},
};

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

/// TODO: generalize for all `Clone`-able?
impl InterpretableFrom<&AddressValue> for AddressValue {
    fn interpret_from(from: &AddressValue, _context: &InterpreterContext) -> Self {
        from.clone()
    }
}
