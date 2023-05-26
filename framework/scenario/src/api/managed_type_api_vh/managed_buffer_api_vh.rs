use multiversx_sc::{
    api::{InvalidSliceError, ManagedBufferApi},
    types::BoxedBytes,
};

use crate::api::VMHooksBackend;

impl ManagedBufferApi for VMHooksBackend {
    fn mb_new_empty(&self) -> Self::ManagedBufferHandle {
        todo!()
    }

    fn mb_new_from_bytes(&self, _bytes: &[u8]) -> Self::ManagedBufferHandle {
        todo!()
    }

    fn mb_len(&self, _handle: Self::ManagedBufferHandle) -> usize {
        todo!()
    }

    fn mb_to_boxed_bytes(&self, _handle: Self::ManagedBufferHandle) -> BoxedBytes {
        todo!()
    }

    fn mb_load_slice(
        &self,
        _source_handle: Self::ManagedBufferHandle,
        _starting_position: usize,
        _dest_slice: &mut [u8],
    ) -> Result<(), InvalidSliceError> {
        todo!()
    }

    fn mb_copy_slice(
        &self,
        _source_handle: Self::ManagedBufferHandle,
        _starting_pos: usize,
        _slice_len: usize,
        _dest_handle: Self::ManagedBufferHandle,
    ) -> Result<(), InvalidSliceError> {
        todo!()
    }

    fn mb_copy_to_slice_pad_right(
        &self,
        _handle: Self::ManagedBufferHandle,
        _destination: &mut [u8],
    ) {
        todo!()
    }

    fn mb_overwrite(&self, _handle: Self::ManagedBufferHandle, _value: &[u8]) {
        todo!()
    }

    fn mb_set_slice(
        &self,
        _dest_handle: Self::ManagedBufferHandle,
        _starting_position: usize,
        _source_slice: &[u8],
    ) -> Result<(), InvalidSliceError> {
        todo!()
    }

    fn mb_set_random(&self, _dest_handle: Self::ManagedBufferHandle, _length: usize) {
        todo!()
    }

    fn mb_append(
        &self,
        _accumulator_handle: Self::ManagedBufferHandle,
        _data_handle: Self::ManagedBufferHandle,
    ) {
        todo!()
    }

    fn mb_append_bytes(&self, _accumulator_handle: Self::ManagedBufferHandle, _bytes: &[u8]) {
        todo!()
    }

    fn mb_eq(
        &self,
        _handle1: Self::ManagedBufferHandle,
        _handle2: Self::ManagedBufferHandle,
    ) -> bool {
        todo!()
    }

    fn mb_to_hex(
        &self,
        _source_handle: Self::ManagedBufferHandle,
        _dest_handle: Self::ManagedBufferHandle,
    ) {
        todo!()
    }
}
