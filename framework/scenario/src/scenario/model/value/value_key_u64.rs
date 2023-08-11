use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
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

impl PartialEq for U64Key {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for U64Key {}

impl PartialOrd for U64Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for U64Key {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl InterpretableFrom<&str> for U64Key {
    fn interpret_from(from: &str, context: &InterpreterContext) -> Self {
        let bytes = interpret_string(from, context);
        let bu = BigUint::from_bytes_be(&bytes);
        U64Key {
            value: bu.to_u64().unwrap(),
            original: from.to_string(),
        }
    }
}

impl IntoRaw<String> for U64Key {
    fn into_raw(self) -> String {
        self.original
    }
}

impl From<u64> for U64Key {
    fn from(from: u64) -> Self {
        U64Key {
            value: from,
            original: from.to_string(),
        }
    }
}

impl From<u32> for U64Key {
    fn from(from: u32) -> Self {
        U64Key {
            value: from as u64,
            original: from.to_string(),
        }
    }
}

impl From<i32> for U64Key {
    fn from(from: i32) -> Self {
        assert!(from >= 0, "U64Key cannot be negative");
        Self::from(from as u32)
    }
}

impl From<&str> for U64Key {
    fn from(from: &str) -> Self {
        U64Key::interpret_from(from, &InterpreterContext::default())
    }
}

impl fmt::Display for U64Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.original.fmt(f)
    }
}
