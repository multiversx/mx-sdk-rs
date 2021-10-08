use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    value_interpreter::interpret_string,
};

use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::{
    cmp::{Ord, Ordering},
    fmt,
};

/// Currently not used.
#[derive(Clone, Debug)]
pub struct U64Key {
    pub value: u64,
    pub original: String,
}

impl From<u64> for U64Key {
    fn from(v: u64) -> Self {
        U64Key {
            value: v,
            original: String::default(),
        }
    }
}

impl PartialEq for U64Key {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for U64Key {}

impl PartialOrd for U64Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for U64Key {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl InterpretableFrom<String> for U64Key {
    fn interpret_from(from: String, context: &InterpreterContext) -> Self {
        let bytes = interpret_string(&from, context);
        let bu = BigUint::from_bytes_be(&bytes);
        U64Key {
            value: bu.to_u64().unwrap(),
            original: from,
        }
    }
}

impl fmt::Display for U64Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.original.fmt(f)
    }
}
