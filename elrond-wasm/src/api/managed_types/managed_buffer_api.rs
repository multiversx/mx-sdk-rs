use crate::abi::TypeAbi;
use crate::types::BoxedBytes;

/// A raw bytes buffer managed by Arwen.
pub trait ManagedBufferApi: Sized + TypeAbi {
	fn new_empty() -> Self;

	fn new_from_bytes(bytes: &[u8]) -> Self;

	fn len(&self) -> usize;

	fn overwrite(&mut self, value: &[u8]);

	fn extend_from_slice(&mut self, slice: &[u8]);

	fn to_boxed_bytes(&self) -> BoxedBytes;
}
