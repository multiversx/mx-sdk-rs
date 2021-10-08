use std::fmt;

use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::ValueSubTree,
    value_interpreter::interpret_subtree,
};

#[derive(PartialEq, Clone, Debug)]
pub struct AddressValue {
    pub value: [u8; 32],
    pub original: ValueSubTree,
}

impl fmt::Display for AddressValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.original.fmt(f)
    }
}

impl InterpretableFrom<ValueSubTree> for AddressValue {
    fn interpret_from(from: ValueSubTree, context: &InterpreterContext) -> Self {
        let bytes = interpret_subtree(&from, context);
        let mut value = [0u8; 32];
        if bytes.len() == 32 {
            value.copy_from_slice(&bytes[..]);
        } else {
            panic!("account address is not 32 bytes in length");
        }
        AddressValue {
            value,
            original: from,
        }
    }
}
