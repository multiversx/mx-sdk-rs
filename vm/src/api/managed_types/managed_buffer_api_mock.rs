use crate::DebugApi;
use multiversx_sc::{
    api::{
        use_raw_handle, HandleConstraints, HandleTypeInfo, InvalidSliceError, ManagedBufferApiImpl,
    },
    types::heap::BoxedBytes,
};

impl DebugApi {
    pub(crate) fn mb_get(&self, handle: <Self as HandleTypeInfo>::ManagedBufferHandle) -> Vec<u8> {
        handle
            .context
            .m_types_borrow()
            .mb_get(handle.get_raw_handle_unchecked())
            .to_vec()
    }

    pub(super) fn mb_set(
        &self,
        handle: <Self as HandleTypeInfo>::ManagedBufferHandle,
        value: Vec<u8>,
    ) {
        handle
            .context
            .m_types_borrow_mut()
            .mb_set(handle.get_raw_handle_unchecked(), value);
    }
}

impl ManagedBufferApiImpl for DebugApi {
    fn mb_new_empty(&self) -> Self::ManagedBufferHandle {
        use_raw_handle(self.m_types_borrow_mut().mb_new(Vec::new()))
    }

    fn mb_new_from_bytes(&self, bytes: &[u8]) -> Self::ManagedBufferHandle {
        use_raw_handle(self.m_types_borrow_mut().mb_new(Vec::from(bytes)))
    }

    fn mb_len(&self, handle: Self::ManagedBufferHandle) -> usize {
        let managed_types = handle.context.m_types_borrow();
        managed_types.mb_len(handle.get_raw_handle_unchecked())
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
        let managed_types = source_handle.context.m_types_borrow();
        managed_types.mb_load_slice(
            source_handle.get_raw_handle_unchecked(),
            starting_position,
            dest_slice,
        )
    }

    fn mb_copy_slice(
        &self,
        source_handle: Self::ManagedBufferHandle,
        starting_position: usize,
        slice_len: usize,
        dest_handle: Self::ManagedBufferHandle,
    ) -> Result<(), InvalidSliceError> {
        // careful: the 2 handles might point to differnt maps
        let slice = source_handle.context.m_types_borrow().mb_get_slice(
            source_handle.get_raw_handle_unchecked(),
            starting_position,
            slice_len,
        )?;
        dest_handle
            .context
            .m_types_borrow_mut()
            .mb_set(dest_handle.get_raw_handle_unchecked(), slice);
        Ok(())
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
        let mut managed_types = dest_handle.context.m_types_borrow_mut();
        managed_types.mb_set_slice(
            dest_handle.get_raw_handle_unchecked(),
            starting_position,
            source_slice,
        )
    }

    fn mb_set_random(&self, dest_handle: Self::ManagedBufferHandle, length: usize) {
        let bytes = self.rng_borrow_mut().next_bytes(length);
        self.mb_set(dest_handle, bytes);
    }

    fn mb_append(
        &self,
        accumulator_handle: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        // careful: the 2 handles might point to differnt maps
        let mut data = self.mb_get(data_handle);
        accumulator_handle.context.m_types_borrow_mut().mb_update(
            accumulator_handle.get_raw_handle_unchecked(),
            |accumulator| {
                accumulator.append(&mut data);
            },
        );
    }

    fn mb_append_bytes(&self, accumulator_handle: Self::ManagedBufferHandle, bytes: &[u8]) {
        accumulator_handle
            .context
            .m_types_borrow_mut()
            .mb_append_bytes(accumulator_handle.get_raw_handle_unchecked(), bytes);
    }

    fn mb_eq(
        &self,
        handle1: Self::ManagedBufferHandle,
        handle2: Self::ManagedBufferHandle,
    ) -> bool {
        // careful: the 2 handles might point to differnt maps
        let bytes1 = self.mb_get(handle1);
        let bytes2 = self.mb_get(handle2);
        bytes1 == bytes2
    }

    fn mb_to_hex(
        &self,
        source_handle: Self::ManagedBufferHandle,
        dest_handle: Self::ManagedBufferHandle,
    ) {
        // careful: the 2 handles might point to differnt maps
        let data = self.mb_get(source_handle);
        let encoded = hex::encode(data);
        self.mb_set(dest_handle, encoded.into_bytes());
    }
}
