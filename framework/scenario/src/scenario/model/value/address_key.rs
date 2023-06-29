use super::{value_from_slice, AddressValue};
use crate::{
    multiversx_sc::types::Address,
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        value_interpreter::interpret_string,
    },
};
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

    pub fn to_vm_address(&self) -> multiversx_chain_vm::types::VMAddress {
        self.value.as_array().into()
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

impl IntoRaw<String> for AddressKey {
    fn into_raw(self) -> String {
        self.original
    }
}

impl From<&str> for AddressKey {
    fn from(from: &str) -> Self {
        Self::interpret_from(from, &InterpreterContext::default())
    }
}

impl From<String> for AddressKey {
    fn from(from: String) -> Self {
        Self::interpret_from(from, &InterpreterContext::default())
    }
}

impl From<&AddressValue> for AddressKey {
    fn from(from: &AddressValue) -> Self {
        AddressKey {
            value: from.to_address(),
            original: from.original.to_concatenated_string(),
        }
    }
}

impl From<AddressValue> for AddressKey {
    fn from(from: AddressValue) -> Self {
        AddressKey::from(&from)
    }
}

impl From<&Address> for AddressKey {
    fn from(from: &Address) -> Self {
        AddressKey {
            value: from.clone(),
            original: format!("0x{}", hex::encode(from)),
        }
    }
}
