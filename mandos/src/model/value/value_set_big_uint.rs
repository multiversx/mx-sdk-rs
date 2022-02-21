use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::ValueSubTree,
    value_interpreter::interpret_subtree,
};

use num_bigint::BigUint;
use std::fmt;

#[derive(Debug)]
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
