use crate::DebugApi;
use multiversx_sc::{
    api::{HandleTypeInfo, InvalidSliceError, ManagedBufferApi},
    types::heap::BoxedBytes,
};

impl DebugApi {
    fn mb_get_slice(
        &self,
        source_handle: <Self as HandleTypeInfo>::ManagedBufferHandle,
        starting_position: usize,
        slice_len: usize,
    ) -> Option<Vec<u8>> {
        let all_bytes = self.mb_get(source_handle);
        if starting_position + slice_len <= all_bytes.len() {
            Some(all_bytes[starting_position..starting_position + slice_len].to_vec())
        } else {
            None
        }
    }

    pub(crate) fn mb_get(&self, handle: <Self as HandleTypeInfo>::ManagedBufferHandle) -> Vec<u8> {
        let managed_types = handle.context.m_types_borrow();
        managed_types
            .managed_buffer_map
            .get(handle.get_raw_handle_unchecked())
            .clone()
    }

    fn mb_update<R, F: FnOnce(&mut Vec<u8>) -> R>(
        &self,
        handle: <Self as HandleTypeInfo>::ManagedBufferHandle,
        f: F,
    ) -> R {
        let mut managed_types = handle.context.m_types_borrow_mut();
        let value = managed_types
            .managed_buffer_map
            .get_mut(handle.get_raw_handle_unchecked());
        f(value)
    }

    fn mb_set(&self, handle: <Self as HandleTypeInfo>::ManagedBufferHandle, value: Vec<u8>) {
        let mut managed_types = handle.context.m_types_borrow_mut();
        managed_types
            .managed_buffer_map
            .insert(handle.get_raw_handle_unchecked(), value);
    }

    fn mb_new(&self, value: Vec<u8>) -> <Self as HandleTypeInfo>::ManagedBufferHandle {
        let mut managed_types = self.m_types_borrow_mut();
        managed_types.managed_buffer_map.insert_new_handle(value)
    }
}

impl ManagedBufferApi for DebugApi {
    fn mb_new_empty(&self) -> Self::ManagedBufferHandle {
        self.mb_new(Vec::new())
    }

    fn mb_new_from_bytes(&self, bytes: &[u8]) -> Self::ManagedBufferHandle {
        self.mb_new(Vec::from(bytes))
    }

    fn mb_len(&self, handle: Self::ManagedBufferHandle) -> usize {
        let data = self.mb_get(handle);
        data.len()
    }

    fn mb_to_boxed_bytes(&self, handle: Self::ManagedBufferHandle) -> BoxedBytes {
        let data = self.mb_get(handle);
        data.into()
    }

    fn mb_load_slice(
        &self,
        source_handle: Self::ManagedBufferHandle,
        starting_position: usize,
        dest_slice: &mut [u8],
    ) -> Result<(), InvalidSliceError> {
        let opt_slice = self.mb_get_slice(source_handle, starting_position, dest_slice.len());
        if let Some(slice) = opt_slice {
            dest_slice.copy_from_slice(slice.as_slice());
            Ok(())
        } else {
            Err(InvalidSliceError)
        }
    }

    fn mb_copy_slice(
        &self,
        source_handle: Self::ManagedBufferHandle,
        starting_position: usize,
        slice_len: usize,
        dest_handle: Self::ManagedBufferHandle,
    ) -> Result<(), InvalidSliceError> {
        let opt_slice = self.mb_get_slice(source_handle, starting_position, slice_len);
        if let Some(slice) = opt_slice {
            self.mb_set(dest_handle, slice);
            Ok(())
        } else {
            Err(InvalidSliceError)
        }
    }

    fn mb_copy_to_slice_pad_right(
        &self,
        handle: Self::ManagedBufferHandle,
        destination: &mut [u8],
    ) {
        let bytes = self.mb_to_boxed_bytes(handle);
        let offset = 32 - bytes.len();
        destination[offset..].copy_from_slice(bytes.as_slice());
    }

    fn mb_overwrite(&self, handle: Self::ManagedBufferHandle, value: &[u8]) {
        self.mb_set(handle, value.into());
    }

    fn mb_set_slice(
        &self,
        dest_handle: Self::ManagedBufferHandle,
        starting_position: usize,
        source_slice: &[u8],
    ) -> Result<(), InvalidSliceError> {
        self.mb_update(dest_handle, |bytes| {
            let end_position = starting_position + source_slice.len();
            if end_position <= bytes.len() {
                bytes[starting_position..end_position].copy_from_slice(source_slice);
                Ok(())
            } else {
                Err(InvalidSliceError)
            }
        })
    }

    fn mb_set_random(&self, dest_handle: Self::ManagedBufferHandle, length: usize) {
        let mut bytes = Vec::<u8>::new();
        bytes.resize(length, 0);
        let mut rng = self.rng_borrow_mut();
        rng.fill(&mut bytes[..]);
        self.mb_set(dest_handle, bytes);
    }

    fn mb_append(
        &self,
        accumulator_handle: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        let mut data = self.mb_get(data_handle);
        self.mb_update(accumulator_handle, |accumulator| {
            accumulator.append(&mut data);
        });
    }

    fn mb_append_bytes(&self, accumulator_handle: Self::ManagedBufferHandle, bytes: &[u8]) {
        self.mb_update(accumulator_handle, |accumulator| {
            accumulator.extend_from_slice(bytes);
        });
    }

    fn mb_eq(
        &self,
        handle1: Self::ManagedBufferHandle,
        handle2: Self::ManagedBufferHandle,
    ) -> bool {
        let bytes1 = self.mb_get(handle1);
        let bytes2 = self.mb_get(handle2);
        bytes1 == bytes2
    }

    fn mb_to_hex(
        &self,
        source_handle: Self::ManagedBufferHandle,
        dest_handle: Self::ManagedBufferHandle,
    ) {
        let data = self.mb_get(source_handle);
        let encoded = hex::encode(data);
        self.mb_set(dest_handle, encoded.into_bytes());
    }
}
