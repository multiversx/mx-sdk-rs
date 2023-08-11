use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    value_interpreter::interpret_string,
};

use std::{
    cmp::{Ord, Ordering},
    fmt,
};

#[derive(Clone, Debug)]
pub struct BytesKey {
    pub value: Vec<u8>,
    pub original: String,
}

impl From<Vec<u8>> for BytesKey {
    fn from(v: Vec<u8>) -> Self {
        BytesKey {
            value: v,
            original: String::default(),
        }
    }
}

impl IntoRaw<String> for BytesKey {
    fn into_raw(self) -> String {
        self.original
    }
}

impl PartialEq for BytesKey {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for BytesKey {}

impl PartialOrd for BytesKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BytesKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl InterpretableFrom<&str> for BytesKey {
    fn interpret_from(from: &str, context: &InterpreterContext) -> Self {
        let bytes = interpret_string(from, context);
        BytesKey {
            value: bytes,
            original: from.to_string(),
        }
    }
}

impl InterpretableFrom<String> for BytesKey {
    fn interpret_from(from: String, context: &InterpreterContext) -> Self {
        let bytes = interpret_string(&from, context);
        BytesKey {
            value: bytes,
            original: from,
        }
    }
}

impl From<&str> for BytesKey {
    fn from(from: &str) -> Self {
        Self::interpret_from(from, &InterpreterContext::default())
    }
}

impl fmt::Display for BytesKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.original.fmt(f)
    }
}
