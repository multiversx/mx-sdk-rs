use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::ValueSubTree,
    value_interpreter::{interpret_string, interpret_subtree},
};

use crate::multiversx_sc::api::ManagedTypeApi;
use num_bigint::BigUint;
use std::fmt;

#[derive(Debug, Clone)]
pub struct BigUintValue {
    pub value: BigUint,
    pub original: ValueSubTree,
}

impl InterpretableFrom<ValueSubTree> for BigUintValue {
    fn interpret_from(from: ValueSubTree, context: &InterpreterContext) -> Self {
        let bytes = interpret_subtree(&from, context);
        BigUintValue {
            value: BigUint::from_bytes_be(&bytes),
            original: from,
        }
    }
}

impl InterpretableFrom<&str> for BigUintValue {
    fn interpret_from(from: &str, context: &InterpreterContext) -> Self {
        let bytes = interpret_string(from, context);
        BigUintValue {
            value: BigUint::from_bytes_be(&bytes),
            original: ValueSubTree::Str(from.to_string()),
        }
    }
}

impl IntoRaw<ValueSubTree> for BigUintValue {
    fn into_raw(self) -> ValueSubTree {
        self.original
    }
}

impl BigUintValue {
    pub fn into_raw_opt(self) -> Option<ValueSubTree> {
        if self.value == 0u32.into() {
            None
        } else {
            Some(self.into_raw())
        }
    }
}

impl From<u32> for BigUintValue {
    fn from(from: u32) -> Self {
        BigUintValue {
            value: from.into(),
            original: ValueSubTree::Str(from.to_string()),
        }
    }
}

impl From<u64> for BigUintValue {
    fn from(from: u64) -> Self {
        BigUintValue {
            value: from.into(),
            original: ValueSubTree::Str(from.to_string()),
        }
    }
}

impl From<u128> for BigUintValue {
    fn from(from: u128) -> Self {
        BigUintValue {
            value: from.into(),
            original: ValueSubTree::Str(from.to_string()),
        }
    }
}

impl From<BigUint> for BigUintValue {
    fn from(from: BigUint) -> Self {
        let s = from.to_string();
        BigUintValue {
            value: from,
            original: ValueSubTree::Str(s),
        }
    }
}

impl From<&BigUint> for BigUintValue {
    fn from(from: &BigUint) -> Self {
        Self::from(from.clone())
    }
}

impl<M: ManagedTypeApi> From<crate::multiversx_sc::types::BigUint<M>> for BigUintValue {
    fn from(from: crate::multiversx_sc::types::BigUint<M>) -> Self {
        let value = BigUint::from_bytes_be(from.to_bytes_be().as_slice());
        BigUintValue {
            original: ValueSubTree::Str(value.to_string()),
            value,
        }
    }
}

impl From<&str> for BigUintValue {
    fn from(from: &str) -> Self {
        BigUintValue::interpret_from(from, &InterpreterContext::default())
    }
}

impl fmt::Display for BigUintValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.original.fmt(f)
    }
}

impl Default for BigUintValue {
    fn default() -> Self {
        BigUintValue {
            original: ValueSubTree::default(),
            value: BigUint::from(0u32),
        }
    }
}
