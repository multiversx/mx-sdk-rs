use std::fmt;

use crate::multiversx_sc::types::Address;

use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::ValueSubTree,
    value_interpreter::{interpret_string, interpret_subtree},
};

use super::AddressKey;

#[derive(PartialEq, Eq, Clone, Debug)]
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

    pub fn to_vm_address(&self) -> multiversx_chain_vm::types::VMAddress {
        self.value.as_array().into()
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

impl IntoRaw<ValueSubTree> for AddressValue {
    fn into_raw(self) -> ValueSubTree {
        self.original
    }
}

impl From<&AddressValue> for AddressValue {
    fn from(from: &AddressValue) -> Self {
        from.clone()
    }
}

impl From<&AddressKey> for AddressValue {
    fn from(from: &AddressKey) -> Self {
        AddressValue {
            value: from.to_address(),
            original: ValueSubTree::Str(from.original.clone()),
        }
    }
}

impl From<AddressKey> for AddressValue {
    fn from(from: AddressKey) -> Self {
        AddressValue::from(&from)
    }
}

impl From<&Address> for AddressValue {
    fn from(from: &Address) -> Self {
        AddressValue {
            value: from.clone(),
            original: ValueSubTree::Str(format!("0x{}", hex::encode(from))),
        }
    }
}

impl From<&str> for AddressValue {
    fn from(from: &str) -> Self {
        AddressValue::interpret_from(from, &InterpreterContext::default())
    }
}
