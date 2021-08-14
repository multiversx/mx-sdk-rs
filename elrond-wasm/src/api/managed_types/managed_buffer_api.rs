use crate::types::BoxedBytes;

use super::Handle;

/// A raw bytes buffer managed by Arwen.
pub trait ManagedBufferApi {
    fn mb_new_empty(&self) -> Handle;

    fn mb_new_from_bytes(&self, bytes: &[u8]) -> Handle;

    fn mb_len(&self, handle: Handle) -> usize;

    fn mb_to_boxed_bytes(&self, handle: Handle) -> BoxedBytes;

    fn mb_load_slice(
        &self,
        source_handle: Handle,
        starting_position: usize,
        dest_slice: &mut [u8],
    ) -> bool;

    fn mb_copy_slice(
        &self,
        source_handle: Handle,
        starting_pos: usize,
        slice_len: usize,
        dest_handle: Handle,
    ) -> bool;

    fn mb_overwrite(&self, handle: Handle, value: &[u8]);

    fn mb_append(&self, accumulator_handle: Handle, data_handle: Handle);

    fn mb_append_bytes(&self, accumulator_handle: Handle, bytes: &[u8]);
}
