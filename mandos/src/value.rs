use super::value_interpreter::*;
use super::value_raw::*;
use super::context::*;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::fmt;
use std::cmp::{Ord, Ordering};

pub trait InterpretableFrom<T> {
    fn interpret_from(from: T, context: &InterpreterContext) -> Self;
}

#[derive(Clone, Debug)]
pub struct BytesValue {
    pub value: Vec<u8>,
    pub original: ValueSubTree,
}

impl BytesValue {
    pub fn empty() -> Self {
        BytesValue {
            value: Vec::new(),
            original: ValueSubTree::Str(String::default())
        }
    }
}

impl From<Vec<u8>> for BytesValue {
    fn from(v: Vec<u8>) -> Self {
        BytesValue {
            value: v,
            original: ValueSubTree::Str(String::default())
        }
    }
}

impl InterpretableFrom<ValueSubTree> for BytesValue {
    fn interpret_from(from: ValueSubTree, context: &InterpreterContext) -> Self {
        BytesValue {
            value: interpret_subtree(&from, context),
            original: from,
        }
    }
}

impl fmt::Display for BytesValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.original.fmt(f)
    }
}

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

#[derive(Debug)]
pub struct U64Value {
    pub value: u64,
    pub original: ValueSubTree,
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

#[derive(Clone, Debug)]
pub struct BytesKey {
    pub value: Vec<u8>,
    pub original: String,
}

impl From<Vec<u8>> for BytesKey {
    fn from(v: Vec<u8>) -> Self {
        BytesKey {
            value: v,
            original: String::default()
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
