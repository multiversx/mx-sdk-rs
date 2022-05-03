use std::fmt;

use elrond_wasm::types::Address;

use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::ValueSubTree,
    value_interpreter::{interpret_string, interpret_subtree},
};

use super::AddressKey;

#[derive(PartialEq, Clone, Debug)]
pub struct AddressValue {
    pub value: Address,
    pub original: ValueSubTree,
}

impl Default for AddressValue {
    fn default() -> Self {
        Self {
            value: Address::zero(),
            original: Default::default(),
        }
    }
}

impl AddressValue {
    pub fn to_address(&self) -> Address {
        self.value.clone()
    }
}

impl fmt::Display for AddressValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.original.fmt(f)
    }
}

pub(crate) fn value_from_slice(slice: &[u8]) -> Address {
    let mut value = [0u8; 32];
    if slice.len() == 32 {
        value.copy_from_slice(slice);
    } else {
        panic!("account address is not 32 bytes in length");
    }
    value.into()
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
            value: from.to_address(),
            original: ValueSubTree::Str(from.original.clone()),
        }
    }
}

impl InterpretableFrom<AddressKey> for AddressValue {
    fn interpret_from(from: AddressKey, context: &InterpreterContext) -> Self {
        AddressValue::interpret_from(&from, context)
    }
}

impl InterpretableFrom<&Address> for AddressValue {
    fn interpret_from(from: &Address, _context: &InterpreterContext) -> Self {
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
