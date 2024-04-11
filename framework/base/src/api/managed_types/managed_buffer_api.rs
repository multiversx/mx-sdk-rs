use crate::types::heap::BoxedBytes;

use super::HandleTypeInfo;

/// Returned if load/copy slice could not be performed.
/// No further data needed.
#[derive(Debug)]
pub struct InvalidSliceError;

/// A raw bytes buffer managed by Arwen.
pub trait ManagedBufferApiImpl: HandleTypeInfo {
    /// Requests a new handle from the VM. No longer used extensively.
    fn mb_new_empty(&self) -> Self::ManagedBufferHandle;

    /// Requests a new handle from the VM, initialized with some data. No longer used extensively.
    fn mb_new_from_bytes(&self, bytes: &[u8]) -> Self::ManagedBufferHandle;

    fn mb_len(&self, handle: Self::ManagedBufferHandle) -> usize;

    fn mb_to_boxed_bytes(&self, handle: Self::ManagedBufferHandle) -> BoxedBytes;

    /// TODO: investigate the impact of using `Result<(), ()>` on the wasm output.
    fn mb_load_slice(
        &self,
        source_handle: Self::ManagedBufferHandle,
        starting_position: usize,
        dest_slice: &mut [u8],
    ) -> Result<(), InvalidSliceError>;

    /// TODO: investigate the impact of using `Result<(), ()>` on the wasm output.
    fn mb_copy_slice(
        &self,
        source_handle: Self::ManagedBufferHandle,
        starting_position: usize,
        slice_len: usize,
        dest_handle: Self::ManagedBufferHandle,
    ) -> Result<(), InvalidSliceError>;

    fn mb_overwrite(&self, handle: Self::ManagedBufferHandle, value: &[u8]);

    fn mb_set_slice(
        &self,
        dest_handle: Self::ManagedBufferHandle,
        starting_position: usize,
        source_slice: &[u8],
    ) -> Result<(), InvalidSliceError>;

    fn mb_set_random(&self, dest_handle: Self::ManagedBufferHandle, length: usize);

    fn mb_append(
        &self,
        accumulator_handle: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    );

    fn mb_append_bytes(&self, accumulator_handle: Self::ManagedBufferHandle, bytes: &[u8]);

    fn mb_eq(&self, handle1: Self::ManagedBufferHandle, handle2: Self::ManagedBufferHandle)
        -> bool;

    fn mb_to_hex(
        &self,
        source_handle: Self::ManagedBufferHandle,
        dest_handle: Self::ManagedBufferHandle,
    );
}
