use super::*;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Eq)]
pub struct AddressKey {
	pub value: [u8; 32],
	pub original: String,
}

impl Ord for AddressKey {
	fn cmp(&self, other: &Self) -> Ordering {
		self.original.cmp(&other.original)
	}
}

impl PartialOrd for AddressKey {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for AddressKey {
	fn eq(&self, other: &Self) -> bool {
		self.original == other.original
	}
}

impl InterpretableFrom<String> for AddressKey {
	fn interpret_from(from: String, context: &InterpreterContext) -> Self {
		let bytes = interpret_string(from.as_str(), context);
		let mut value = [0u8; 32];
		if bytes.len() == 32 {
			value.copy_from_slice(&bytes[..]);
		}
		AddressKey {
			value,
			original: from,
		}
	}
}

impl fmt::Display for AddressKey {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.original.fmt(f)
	}
}

#[derive(PartialEq, Clone, Debug)]
pub struct AddressValue {
	pub value: [u8; 32],
	pub original: ValueSubTree,
}

impl InterpretableFrom<ValueSubTree> for AddressValue {
	fn interpret_from(from: ValueSubTree, context: &InterpreterContext) -> Self {
		let bytes = interpret_subtree(&from, context);
		let mut value = [0u8; 32];
		if bytes.len() == 32 {
			value.copy_from_slice(&bytes[..]);
		}
		AddressValue {
			value,
			original: from,
		}
	}
}

impl fmt::Display for AddressValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.original.fmt(f)
	}
}
