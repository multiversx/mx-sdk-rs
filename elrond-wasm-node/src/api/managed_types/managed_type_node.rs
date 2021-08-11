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

impl elrond_wasm::api::ManagedTypeApi for crate::ArwenApiImpl {
    fn managed_buffer_to_big_int_signed(&self, buffer_handle: Handle) -> Handle {
        unsafe {
            let big_int_handle = bigIntNew(0);
            mBufferToBigIntSigned(buffer_handle, big_int_handle);
            big_int_handle
        }
    }

    fn big_int_to_managed_buffer_signed(&self, big_int_handle: Handle) -> Handle {
        unsafe {
            let buffer_handle = mBufferNew();
            mBufferToBigIntSigned(buffer_handle, big_int_handle);
            buffer_handle
        }
    }
}
