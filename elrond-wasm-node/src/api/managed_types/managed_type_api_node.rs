use elrond_wasm::api::Handle;

#[allow(dead_code)]
extern "C" {
    fn mBufferNew() -> i32;
    fn bigIntNew(value: i64) -> i32;

    fn mBufferToBigIntUnsigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
    fn mBufferToBigIntSigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
    fn mBufferFromBigIntUnsigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
    fn mBufferFromBigIntSigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
}

impl elrond_wasm::api::ManagedTypeApi for crate::VmApiImpl {
    fn instance() -> Self {
        crate::VmApiImpl {}
    }

    #[inline]
    fn mb_to_big_int_unsigned(&self, buffer_handle: Handle) -> Handle {
        unsafe {
            let big_int_handle = bigIntNew(0);
            mBufferToBigIntUnsigned(buffer_handle, big_int_handle);
            big_int_handle
        }
    }

    #[inline]
    fn mb_to_big_int_signed(&self, buffer_handle: Handle) -> Handle {
        unsafe {
            let big_int_handle = bigIntNew(0);
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
}
