use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::ValueSubTree,
    value_interpreter::{interpret_string, interpret_subtree},
};

use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::fmt;

#[derive(Debug)]
pub struct U64Value {
    pub value: u64,
    pub original: ValueSubTree,
}

impl U64Value {
    pub fn empty() -> Self {
        U64Value {
            value: 0,
            original: ValueSubTree::Str(String::default()),
        }
    }

    pub fn zero() -> Self {
        U64Value {
            value: 0,
            original: ValueSubTree::Str("0".to_string()),
        }
    }
}

impl Default for U64Value {
    fn default() -> Self {
        U64Value::empty()
    }
}

impl InterpretableFrom<ValueSubTree> for U64Value {
    fn interpret_from(from: ValueSubTree, context: &InterpreterContext) -> Self {
        let bytes = interpret_subtree(&from, context);
        let bu = BigUint::from_bytes_be(&bytes);
        U64Value {
            value: bu.to_u64().unwrap(),
            original: from,
        }
    }
}

impl IntoRaw<ValueSubTree> for U64Value {
    fn into_raw(self) -> ValueSubTree {
        self.original
    }
}

impl U64Value {
    pub fn into_raw_opt(self) -> Option<ValueSubTree> {
        if self.value > 0 {
            Some(self.into_raw())
        } else {
            None
        }
    }
}

impl InterpretableFrom<&str> for U64Value {
    fn interpret_from(from: &str, context: &InterpreterContext) -> Self {
        let bytes = interpret_string(from, context);
        let bu = BigUint::from_bytes_be(&bytes);
        U64Value {
            value: bu.to_u64().unwrap(),
            original: ValueSubTree::Str(from.to_string()),
        }
    }
}

impl InterpretableFrom<u64> for U64Value {
    fn interpret_from(from: u64, _context: &InterpreterContext) -> Self {
        U64Value {
            value: from,
            original: ValueSubTree::Str(from.to_string()),
        }
    }
}

impl fmt::Display for U64Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.original.fmt(f)
    }
}
