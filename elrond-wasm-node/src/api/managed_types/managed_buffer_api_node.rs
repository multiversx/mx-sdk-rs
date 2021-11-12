use crate::{api::unsafe_buffer, error_hook};
use elrond_wasm::{
    api::{Handle, InvalidSliceError, ManagedBufferApi},
    err_msg,
    types::{BoxedBytes, ManagedAddress, ManagedType, TokenIdentifier},
};

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
    #[cfg(not(feature = "unmanaged-ei"))]
    fn mBufferEq(handle1: i32, handle2: i32) -> i32;
    fn mBufferSetBytes(mBufferHandle: i32, byte_ptr: *const u8, byte_len: i32) -> i32;
    fn mBufferAppend(accumulatorHandle: i32, dataHandle: i32) -> i32;
    fn mBufferAppendBytes(accumulatorHandle: i32, byte_ptr: *const u8, byte_len: i32) -> i32;
}

impl ManagedBufferApi for crate::VmApiImpl {
    #[inline]
    fn mb_new_empty(&self) -> Handle {
        unsafe { mBufferNew() }
    }

    #[inline]
    fn mb_new_from_bytes(&self, bytes: &[u8]) -> Handle {
        unsafe { mBufferNewFromBytes(bytes.as_ptr(), bytes.len() as i32) }
    }

    #[inline]
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

    #[inline]
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

    fn mb_copy_to_slice_pad_right(&self, handle: Handle, destination: &mut [u8]) {
        unsafe {
            let byte_len = mBufferGetLength(handle) as usize;
            if byte_len > destination.len() {
                error_hook::signal_error(err_msg::VALUE_EXCEEDS_SLICE)
            }
            if byte_len > 0 {
                let start_index = destination.len() - byte_len;
                let _ = mBufferGetBytes(handle, destination.as_mut_ptr().add(start_index));
            }
        }
    }

    #[inline]
    fn mb_overwrite(&self, handle: Handle, bytes: &[u8]) {
        unsafe {
            let _ = mBufferSetBytes(handle as i32, bytes.as_ptr(), bytes.len() as i32);
        }
    }

    #[inline]
    fn mb_append(&self, accumulator_handle: Handle, data_handle: Handle) {
        unsafe {
            let _ = mBufferAppend(accumulator_handle as i32, data_handle as i32);
        }
    }

    #[inline]
    fn mb_append_bytes(&self, accumulator_handle: Handle, bytes: &[u8]) {
        unsafe {
            let _ = mBufferAppendBytes(
                accumulator_handle as i32,
                bytes.as_ptr(),
                bytes.len() as i32,
            );
        }
    }

    #[cfg(feature = "unmanaged-ei")]
    fn mb_eq(&self, handle1: Handle, handle2: Handle) -> bool {
        // TODO: might be worth adding a new hook to Arwen for this
        unsafe {
            let len1 = mBufferGetLength(handle1 as i32) as usize;
            let len2 = mBufferGetLength(handle2 as i32) as usize;
            if len1 != len2 {
                return false;
            }
            if len1 == 0 {
                return true;
            }
            let mut bytes1 = BoxedBytes::allocate(len1);
            let mut bytes2 = BoxedBytes::allocate(len2);
            let _ = mBufferGetBytes(handle1, bytes1.as_mut_ptr());
            let _ = mBufferGetBytes(handle2, bytes2.as_mut_ptr());
            bytes1 == bytes2
        }
    }

    #[cfg(not(feature = "unmanaged-ei"))]
    fn mb_eq(&self, handle1: Handle, handle2: Handle) -> bool {
        unsafe { mBufferEq(handle1, handle2) > 0 }
    }
}

pub(crate) unsafe fn unsafe_buffer_load_address(
    address: &ManagedAddress<crate::VmApiImpl>,
) -> *const u8 {
    let unsafe_buffer_1_ptr = unsafe_buffer::buffer_1_ptr();
    let _ = mBufferGetBytes(address.get_raw_handle(), unsafe_buffer_1_ptr);
    unsafe_buffer_1_ptr
}

/// We usually need it at the same time with the address,
/// so we put in in buffer #2.
pub(crate) unsafe fn unsafe_buffer_load_token_identifier(
    token: &TokenIdentifier<crate::VmApiImpl>,
) -> *const u8 {
    let unsafe_buffer_2_ptr = unsafe_buffer::buffer_2_ptr();
    let _ = mBufferGetBytes(token.get_raw_handle(), unsafe_buffer_2_ptr);
    unsafe_buffer_2_ptr
}
