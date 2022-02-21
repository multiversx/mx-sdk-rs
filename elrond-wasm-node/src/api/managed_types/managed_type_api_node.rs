use crate::VmApiImpl;
use elrond_wasm::api::{Handle, ManagedTypeApi, ManagedTypeApiImpl, StaticVarApiImpl};

#[allow(dead_code)]
extern "C" {
    fn mBufferNew() -> i32;
    #[allow(dead_code)]
    fn bigIntNew(value: i64) -> i32;

    fn mBufferToBigIntUnsigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
    fn mBufferToBigIntSigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
    fn mBufferFromBigIntUnsigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
    fn mBufferFromBigIntSigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
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
    fn mb_to_big_int_unsigned(&self, buffer_handle: Handle) -> Handle {
        unsafe {
            let big_int_handle = self.get_next_bigint_handle();
            mBufferToBigIntUnsigned(buffer_handle, big_int_handle);
            big_int_handle
        }
    }

    #[inline]
    fn mb_to_big_int_signed(&self, buffer_handle: Handle) -> Handle {
        unsafe {
            let big_int_handle = self.get_next_bigint_handle();
            mBufferToBigIntSigned(buffer_handle, big_int_handle);
            big_int_handle
        }
    }

    #[inline]
    fn mb_from_big_int_unsigned(&self, big_int_handle: Handle) -> Handle {
        unsafe {
            let buffer_handle = mBufferNew();
            mBufferFromBigIntUnsigned(buffer_handle, big_int_handle);
            buffer_handle
        }
    }

    #[inline]
    fn mb_from_big_int_signed(&self, big_int_handle: Handle) -> Handle {
        unsafe {
            let buffer_handle = mBufferNew();
            mBufferFromBigIntSigned(buffer_handle, big_int_handle);
            buffer_handle
        }
    }

    #[cfg(feature = "vm-validate-token-identifier")]
    fn validate_token_identifier(&self, token_id_handle: Handle) -> bool {
        unsafe { validateTokenIdentifier(token_id_handle) != 0 }
    }
}
