use crate::{
    api::{Handle, ManagedBufferApi},
    types::BoxedBytes,
};

impl ManagedBufferApi for super::UncallableApi {
    fn mb_new_empty(&self) -> Handle {
        unreachable!()
    }

    fn mb_new_from_bytes(&self, _bytes: &[u8]) -> Handle {
        unreachable!()
    }

    fn mb_len(&self, _handle: Handle) -> usize {
        unreachable!()
    }

    fn mb_to_boxed_bytes(&self, _handle: Handle) -> BoxedBytes {
        unreachable!()
    }

    fn mb_load_slice(
        &self,
        _source_handle: Handle,
        _starting_position: usize,
        _dest_slice: &mut [u8],
    ) -> bool {
        unreachable!()
    }

    fn mb_copy_slice(
        &self,
        _source_handle: Handle,
        _starting_pos: usize,
        _slice_len: usize,
        _dest_handle: Handle,
    ) -> bool {
        unreachable!()
    }

    fn mb_overwrite(&self, _handle: Handle, _value: &[u8]) {
        unreachable!()
    }

    fn mb_append(&self, _accumulator_handle: Handle, _data_handle: Handle) {
        unreachable!()
    }

    fn mb_append_bytes(&self, _accumulator_handle: Handle, _bytes: &[u8]) {
        unreachable!()
    }
}
