use crate::types::BoxedBytes;

use super::Handle;

/// Returned if load/copy slice could not be performed.
/// No further data needed.
pub struct InvalidSliceError;

/// A raw bytes buffer managed by Arwen.
pub trait ManagedBufferApi {
    fn mb_new_empty(&self) -> Handle;

    fn mb_new_from_bytes(&self, bytes: &[u8]) -> Handle;

    fn mb_len(&self, handle: Handle) -> usize;

    fn mb_to_boxed_bytes(&self, handle: Handle) -> BoxedBytes;

    /// TODO: investigate the impact of using `Result<(), ()>` on the wasm output.
    fn mb_load_slice(
        &self,
        source_handle: Handle,
        starting_position: usize,
        dest_slice: &mut [u8],
    ) -> Result<(), InvalidSliceError>;

    /// TODO: investigate the impact of using `Result<(), ()>` on the wasm output.
    fn mb_copy_slice(
        &self,
        source_handle: Handle,
        starting_position: usize,
        slice_len: usize,
        dest_handle: Handle,
    ) -> Result<(), InvalidSliceError>;

    fn mb_copy_to_slice_pad_right(&self, handle: Handle, destination: &mut [u8]);

    fn mb_overwrite(&self, handle: Handle, value: &[u8]);

    fn mb_append(&self, accumulator_handle: Handle, data_handle: Handle);

    fn mb_append_bytes(&self, accumulator_handle: Handle, bytes: &[u8]);

    fn mb_eq(&self, handle1: Handle, handle2: Handle) -> bool;
}
