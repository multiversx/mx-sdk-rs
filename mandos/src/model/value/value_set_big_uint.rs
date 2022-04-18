use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::ValueSubTree,
    value_interpreter::{interpret_string, interpret_subtree},
};

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

impl InterpretableFrom<u32> for BigUintValue {
    fn interpret_from(from: u32, _context: &InterpreterContext) -> Self {
        let bytes = from.to_be_bytes().to_vec();
        BigUintValue {
            value: BigUint::from_bytes_be(&bytes),
            original: ValueSubTree::Str(from.to_string()),
        }
    }
}

impl InterpretableFrom<u64> for BigUintValue {
    fn interpret_from(from: u64, _context: &InterpreterContext) -> Self {
        let bytes = from.to_be_bytes().to_vec();
        BigUintValue {
            value: BigUint::from_bytes_be(&bytes),
            original: ValueSubTree::Str(from.to_string()),
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
