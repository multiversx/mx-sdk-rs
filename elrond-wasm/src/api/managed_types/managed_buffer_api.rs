use crate::types::BoxedBytes;

use super::Handle;

/// A raw bytes buffer managed by Arwen.
pub trait ManagedBufferApi {
    fn new_empty(&self) -> Handle;

    fn new_from_bytes(&self, bytes: &[u8]) -> Handle;

    fn len(&self, handle: Handle) -> usize;

    fn overwrite(&self, handle: Handle, value: &[u8]);

    fn extend_from_slice(&self, handle: Handle, slice: &[u8]);

    fn to_boxed_bytes(&self, handle: Handle) -> BoxedBytes;
}
