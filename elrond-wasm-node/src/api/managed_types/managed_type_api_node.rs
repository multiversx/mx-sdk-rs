use crate::VmApiImpl;
use elrond_wasm::api::{Handle, ManagedTypeApi, ManagedTypeApiImpl};

extern "C" {
    fn mBufferToBigIntUnsigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
    fn mBufferToBigIntSigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
    fn mBufferFromBigIntUnsigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
    fn mBufferFromBigIntSigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
    fn mBufferToBigFloat(mBufferHandle: i32, bigFloatHandle: i32) -> i32;
    fn mBufferFromBigFloat(mBufferHandle: i32, bigFloatHandle: i32) -> i32;

    fn validateTokenIdentifier(token_id_handle: i32) -> i32;
}

impl ManagedTypeApi for VmApiImpl {
    type ManagedTypeApiImpl = VmApiImpl;

    fn managed_type_impl() -> Self {
        VmApiImpl {}
    }
}

impl ManagedTypeApiImpl for VmApiImpl {
    #[inline]
    fn mb_to_big_int_unsigned(&self, buffer_handle: Handle, big_int_handle: Handle) {
        unsafe {
            mBufferToBigIntUnsigned(buffer_handle, big_int_handle);
        }
    }

    #[inline]
    fn mb_to_big_int_signed(&self, buffer_handle: Handle, big_int_handle: Handle) {
        unsafe {
            mBufferToBigIntSigned(buffer_handle, big_int_handle);
        }
    }

    #[inline]
    fn mb_from_big_int_unsigned(&self, big_int_handle: Handle, buffer_handle: Handle) {
        unsafe {
            mBufferFromBigIntUnsigned(buffer_handle, big_int_handle);
        }
    }

    #[inline]
    fn mb_from_big_int_signed(&self, big_int_handle: Handle, buffer_handle: Handle) {
        unsafe {
            mBufferFromBigIntSigned(buffer_handle, big_int_handle);
        }
    }

    #[inline]
    fn mb_to_big_float(&self, buffer_handle: Handle, big_float_handle: Handle) {
        unsafe {
            mBufferToBigFloat(buffer_handle, big_float_handle);
        }
    }

    #[inline]
    fn mb_from_big_float(&self, big_float_handle: Handle, buffer_handle: Handle) {
        unsafe {
            mBufferFromBigFloat(buffer_handle, big_float_handle);
        }
    }

    #[inline]
    fn validate_token_identifier(&self, token_id_handle: Handle) -> bool {
        unsafe { validateTokenIdentifier(token_id_handle) != 0 }
    }
}
