use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::ValueSubTree,
    value_interpreter::{interpret_string, interpret_subtree},
};

use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::fmt;

#[derive(Debug, Clone)]
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

impl From<u64> for U64Value {
    fn from(from: u64) -> Self {
        U64Value {
            value: from,
            original: ValueSubTree::Str(from.to_string()),
        }
    }
}

impl From<u32> for U64Value {
    fn from(from: u32) -> Self {
        U64Value {
            value: from as u64,
            original: ValueSubTree::Str(from.to_string()),
        }
    }
}

impl From<i32> for U64Value {
    fn from(from: i32) -> Self {
        assert!(from >= 0, "U64Value cannot be negative");
        Self::from(from as u32)
    }
}

impl From<&str> for U64Value {
    fn from(from: &str) -> Self {
        U64Value::interpret_from(from, &InterpreterContext::default())
    }
}

impl fmt::Display for U64Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.original.fmt(f)
    }
}
