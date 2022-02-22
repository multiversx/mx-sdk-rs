use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::ValueSubTree,
    value_interpreter::interpret_subtree,
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

impl fmt::Display for U64Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.original.fmt(f)
    }
}
