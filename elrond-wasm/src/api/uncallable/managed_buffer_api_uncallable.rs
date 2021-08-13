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

    fn mb_overwrite(&self, _handle: Handle, _value: &[u8]) {
        unreachable!()
    }

    fn mb_append(&self, _accumulator_handle: Handle, _data_handle: Handle) {
        unreachable!()
    }

    fn mb_append_bytes(&self, _accumulator_handle: Handle, _bytes: &[u8]) {
        unreachable!()
    }

    fn mb_to_boxed_bytes(&self, _handle: Handle) -> BoxedBytes {
        unreachable!()
    }
}
