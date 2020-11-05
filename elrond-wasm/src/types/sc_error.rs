use alloc::boxed::Box;
use alloc::vec::Vec;
use elrond_codec::EncodeError;

/// Contains a smart contract execution error message.
///
/// Implemented as a simple boxed slice, for performance reasons.
#[derive(Debug, PartialEq, Eq)]
pub struct SCError(Box<[u8]>);

impl SCError {
	#[inline]
	pub fn as_bytes(&self) -> &[u8] {
		&*self.0
	}
}

impl From<&str> for SCError {
	#[inline]
	fn from(s: &str) -> Self {
		SCError(Box::from(s.as_bytes()))
	}
}

impl From<&[u8]> for SCError {
	#[inline]
	fn from(byte_slice: &[u8]) -> Self {
		SCError(Box::from(byte_slice))
	}
}

impl From<Vec<u8>> for SCError {
	#[inline]
	fn from(v: Vec<u8>) -> Self {
		SCError(v.into_boxed_slice())
	}
}

impl From<EncodeError> for SCError {
	#[inline]
	fn from(err: EncodeError) -> Self {
		SCError::from(err.message_bytes())
	}
}
