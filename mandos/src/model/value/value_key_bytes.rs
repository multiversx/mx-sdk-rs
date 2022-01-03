use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
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

impl PartialEq for BytesKey {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for BytesKey {}

impl PartialOrd for BytesKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for BytesKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
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

impl fmt::Display for BytesKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.original.fmt(f)
    }
}
