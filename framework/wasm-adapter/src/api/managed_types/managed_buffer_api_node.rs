use crate::api::unsafe_buffer;
use multiversx_sc::{
    api::{InvalidSliceError, ManagedBufferApiImpl},
    types::heap::BoxedBytes,
};

#[allow(dead_code)]
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
    fn mBufferEq(handle1: i32, handle2: i32) -> i32;
    fn mBufferSetBytes(mBufferHandle: i32, byte_ptr: *const u8, byte_len: i32) -> i32;

    fn mBufferSetByteSlice(
        mBufferHandle: i32,
        startingPosition: i32,
        dataLength: i32,
        dataOffset: *const u8,
    ) -> i32;

    fn mBufferSetRandom(destinationHandle: i32, length: i32) -> i32;
    fn mBufferAppend(accumulatorHandle: i32, dataHandle: i32) -> i32;
    fn mBufferAppendBytes(accumulatorHandle: i32, byte_ptr: *const u8, byte_len: i32) -> i32;

    fn managedBufferToHex(sourceHandle: i32, destinationHandle: i32);
}

impl ManagedBufferApiImpl for crate::api::VmApiImpl {
    #[inline]
    fn mb_new_empty(&self) -> Self::ManagedBufferHandle {
        unsafe { mBufferNew() }
    }

    #[inline]
    fn mb_new_from_bytes(&self, bytes: &[u8]) -> Self::ManagedBufferHandle {
        unsafe { mBufferNewFromBytes(bytes.as_ptr(), bytes.len() as i32) }
    }

    #[inline]
    fn mb_len(&self, handle: Self::ManagedBufferHandle) -> usize {
        unsafe { mBufferGetLength(handle) as usize }
    }

    fn mb_to_boxed_bytes(&self, handle: Self::ManagedBufferHandle) -> BoxedBytes {
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
        source_handle: Self::ManagedBufferHandle,
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

    #[inline]
    fn mb_copy_slice(
        &self,
        source_handle: Self::ManagedBufferHandle,
        starting_pos: usize,
        slice_len: usize,
        dest_handle: Self::ManagedBufferHandle,
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

    #[inline]
    fn mb_overwrite(&self, handle: Self::ManagedBufferHandle, bytes: &[u8]) {
        unsafe {
            let _ = mBufferSetBytes(handle, bytes.as_ptr(), bytes.len() as i32);
        }
    }

    #[inline]
    fn mb_set_slice(
        &self,
        dest_handle: Self::ManagedBufferHandle,
        starting_position: usize,
        source_slice: &[u8],
    ) -> Result<(), InvalidSliceError> {
        unsafe {
            let err = mBufferSetByteSlice(
                dest_handle,
                starting_position as i32,
                source_slice.len() as i32,
                source_slice.as_ptr(),
            );
            if err == 0 {
                Ok(())
            } else {
                Err(InvalidSliceError)
            }
        }
    }

    #[inline]
    fn mb_set_random(&self, dest_handle: Self::ManagedBufferHandle, length: usize) {
        unsafe {
            let _ = mBufferSetRandom(dest_handle, length as i32);
        }
    }

    #[inline]
    fn mb_append(
        &self,
        accumulator_handle: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            let _ = mBufferAppend(accumulator_handle, data_handle);
        }
    }

    #[inline]
    fn mb_append_bytes(&self, accumulator_handle: Self::ManagedBufferHandle, bytes: &[u8]) {
        unsafe {
            let _ = mBufferAppendBytes(accumulator_handle, bytes.as_ptr(), bytes.len() as i32);
        }
    }

    fn mb_eq(
        &self,
        handle1: Self::ManagedBufferHandle,
        handle2: Self::ManagedBufferHandle,
    ) -> bool {
        unsafe { mBufferEq(handle1, handle2) > 0 }
    }

    fn mb_to_hex(
        &self,
        source_handle: Self::ManagedBufferHandle,
        dest_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            managedBufferToHex(source_handle, dest_handle);
        }
    }
}

pub(crate) unsafe fn unsafe_buffer_load_address(address_handle: i32) -> *const u8 {
    let unsafe_buffer_1_ptr = unsafe_buffer::buffer_1_ptr();
    let _ = mBufferGetBytes(address_handle, unsafe_buffer_1_ptr);
    unsafe_buffer_1_ptr
}

/// We usually need it at the same time with the address,
/// so we put in in buffer #2.
pub(crate) unsafe fn unsafe_buffer_load_token_identifier(token_handle: i32) -> *const u8 {
    let unsafe_buffer_2_ptr = unsafe_buffer::buffer_2_ptr();
    let _ = mBufferGetBytes(token_handle, unsafe_buffer_2_ptr);
    unsafe_buffer_2_ptr
}
