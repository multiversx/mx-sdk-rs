use crate::types::BoxedBytes;
use alloc::vec::Vec;
use elrond_codec::EncodeError;

/// Contains a smart contract execution error message.
///
/// Implemented as a simple boxed slice, for performance reasons.
#[derive(Debug, PartialEq, Eq)]
pub struct SCError(BoxedBytes);

impl SCError {
	#[inline]
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_slice()
	}
}

impl From<BoxedBytes> for SCError {
	#[inline]
	fn from(boxed_bytes: BoxedBytes) -> Self {
		SCError(boxed_bytes)
	}
}

impl From<&str> for SCError {
	#[inline]
	fn from(s: &str) -> Self {
		SCError(BoxedBytes::from(s.as_bytes()))
	}
}

impl From<&[u8]> for SCError {
	#[inline]
	fn from(byte_slice: &[u8]) -> Self {
		SCError(BoxedBytes::from(byte_slice))
	}
}

impl From<Vec<u8>> for SCError {
	#[inline]
	fn from(v: Vec<u8>) -> Self {
		SCError(BoxedBytes::from(v))
	}
}

impl From<EncodeError> for SCError {
	#[inline]
	fn from(err: EncodeError) -> Self {
		SCError::from(err.message_bytes())
	}
}
