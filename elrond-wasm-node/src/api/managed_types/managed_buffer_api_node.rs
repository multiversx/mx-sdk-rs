use elrond_wasm::api::{Handle, InvalidSliceError, ManagedBufferApi};
use elrond_wasm::types::BoxedBytes;

// #[allow(dead_code)]
extern "C" {
    fn mBufferNew() -> i32;
    fn mBufferNewFromBytes(byte_ptr: *const u8, byte_len: i32) -> i32;
    fn mBufferGetLength(mBufferHandle: i32) -> i32;
    fn mBufferGetBytes(mBufferHandle: i32, resultOffset: *mut u8) -> i32;
    fn mBufferGetByteSlice(
        sourceHandle: i32,
        startingPosition: i32,
        sliceLength: i32,
        resultOffset: *mut u8,
    ) -> i32;
    fn mBufferCopyByteSlice(
        sourceHandle: i32,
        startingPosition: i32,
        sliceLength: i32,
        destinationHandle: i32,
    ) -> i32;
    fn mBufferSetBytes(mBufferHandle: i32, byte_ptr: *const u8, byte_len: i32) -> i32;
    fn mBufferAppend(accumulatorHandle: i32, dataHandle: i32) -> i32;
    fn mBufferAppendBytes(accumulatorHandle: i32, byte_ptr: *const u8, byte_len: i32) -> i32;
}

impl ManagedBufferApi for crate::ArwenApiImpl {
    fn mb_new_empty(&self) -> Handle {
        unsafe { mBufferNew() }
    }

    fn mb_new_from_bytes(&self, bytes: &[u8]) -> Handle {
        unsafe { mBufferNewFromBytes(bytes.as_ptr(), bytes.len() as i32) }
    }

    fn mb_len(&self, handle: Handle) -> usize {
        unsafe { mBufferGetLength(handle as i32) as usize }
    }

    fn mb_to_boxed_bytes(&self, handle: Handle) -> BoxedBytes {
        unsafe {
            let len = mBufferGetLength(handle);
            let mut res = BoxedBytes::allocate(len as usize);
            if len > 0 {
                let _ = mBufferGetBytes(handle, res.as_mut_ptr());
            }
            res
        }
    }

    fn mb_load_slice(
        &self,
        source_handle: Handle,
        starting_position: usize,
        dest_slice: &mut [u8],
    ) -> Result<(), InvalidSliceError> {
        unsafe {
            let err = mBufferGetByteSlice(
                source_handle,
                starting_position as i32,
                dest_slice.len() as i32,
                dest_slice.as_mut_ptr(),
            );
            if err == 0 {
                Ok(())
            } else {
                Err(InvalidSliceError)
            }
        }
    }

    fn mb_copy_slice(
        &self,
        source_handle: Handle,
        starting_pos: usize,
        slice_len: usize,
        dest_handle: Handle,
    ) -> Result<(), InvalidSliceError> {
        unsafe {
            let err = mBufferCopyByteSlice(
                source_handle,
                starting_pos as i32,
                slice_len as i32,
                dest_handle,
            );
            if err == 0 {
                Ok(())
            } else {
                Err(InvalidSliceError)
            }
        }
    }

    fn mb_overwrite(&self, handle: Handle, bytes: &[u8]) {
        unsafe {
            let _ = mBufferSetBytes(handle as i32, bytes.as_ptr(), bytes.len() as i32);
        }
    }

    fn mb_append(&self, accumulator_handle: Handle, data_handle: Handle) {
        unsafe {
            let _ = mBufferAppend(accumulator_handle as i32, data_handle as i32);
        }
    }

    fn mb_append_bytes(&self, accumulator_handle: Handle, bytes: &[u8]) {
        unsafe {
            let _ = mBufferAppendBytes(
                accumulator_handle as i32,
                bytes.as_ptr(),
                bytes.len() as i32,
            );
        }
    }
}
