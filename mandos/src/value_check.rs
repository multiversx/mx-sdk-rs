use super::context::*;
use super::value::*;
use super::value_raw::*;
use num_bigint::BigUint;
use std::fmt;

pub trait Checkable<V> {
	fn check(&self, value: V) -> bool;
}

impl Checkable<&[u8]> for BytesValue {
	fn check(&self, value: &[u8]) -> bool {
		self.value.as_slice() == value
	}
}

impl Checkable<&BigUint> for BigUintValue {
	fn check(&self, value: &BigUint) -> bool {
		&self.value == value
	}
}

impl Checkable<u64> for U64Value {
	fn check(&self, value: u64) -> bool {
		self.value == value
	}
}

#[derive(Debug)]
pub enum CheckValue<T> {
	DefaultStar,
	Star,
	Equal(T),
}

impl<T: InterpretableFrom<ValueSubTree>> CheckValue<T> {
	pub fn is_star(&self) -> bool {
		matches!(self, CheckValue::Star | CheckValue::DefaultStar)
	}

	pub fn is_default_star(&self) -> bool {
		matches!(self, CheckValue::DefaultStar)
	}
}

impl<T: InterpretableFrom<ValueSubTree>> Default for CheckValue<T> {
	fn default() -> Self {
		CheckValue::DefaultStar
	}
}

impl<T: InterpretableFrom<ValueSubTree>> InterpretableFrom<ValueSubTree> for CheckValue<T> {
	fn interpret_from(from: ValueSubTree, context: &InterpreterContext) -> Self {
		if let ValueSubTree::Str(s) = &from {
			if s.is_empty() {
				return CheckValue::DefaultStar;
			} else if s == "*" {
				return CheckValue::Star;
			}
		}

		CheckValue::Equal(T::interpret_from(from, context))
	}
}

impl<T: fmt::Display> fmt::Display for CheckValue<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			CheckValue::Star | CheckValue::DefaultStar => write!(f, "*"),
			CheckValue::Equal(eq_value) => eq_value.fmt(f),
		}
	}
}

impl<V, T> Checkable<V> for CheckValue<T>
where
	T: Checkable<V>,
{
	fn check(&self, value: V) -> bool {
		match self {
			CheckValue::DefaultStar | CheckValue::Star => true,
			CheckValue::Equal(eq) => eq.check(value),
		}
	}
}

impl Checkable<&[Vec<u8>]> for Vec<CheckValue<BytesValue>> {
	fn check(&self, values: &[Vec<u8>]) -> bool {
		if self.len() != values.len() {
			return false;
		}
		for (i, cv) in self.iter().enumerate() {
			if !cv.check(values[i].as_slice()) {
				return false;
			}
		}
		true
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_bytes() {
		let bv = BytesValue {
			value: b"abc".to_vec(),
			original: ValueSubTree::Str("abc".to_string()),
		};
		assert!(bv.check(&b"abc"[..]));

		let cb_eq = CheckValue::Equal(bv);
		assert!(cb_eq.check(&b"abc"[..]));

		let cb_star: CheckValue<BytesValue> = CheckValue::Star;
		assert!(cb_star.check(&b"anything_really"[..]));
	}

	#[test]
	fn check_u64() {
		let u64v = U64Value {
			value: 123,
			original: ValueSubTree::Str("123".to_string()),
		};
		assert!(u64v.check(123u64));

		let cb_eq = CheckValue::Equal(u64v);
		assert!(cb_eq.check(123u64));

		let cb_star: CheckValue<U64Value> = CheckValue::Star;
		assert!(cb_star.check(1234567890));
	}
}
