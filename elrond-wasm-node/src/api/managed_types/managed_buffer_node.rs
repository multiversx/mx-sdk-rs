use elrond_wasm::api::{Handle, ManagedBufferApi};
use elrond_wasm::types::BoxedBytes;

// extern int32_t	mBufferNew(void* context);
// extern int32_t 	mBufferNewFromBytes(int32_t dataOffset, int32_t dataLength);
// extern int32_t	mBufferSetBytes(mBufferHandle: i32, int32_t dataOffset, int32_t dataLength);
// extern int32_t 	mBufferGetLength(mBufferHandle: i32);
// extern int32_t	mBufferGetBytes(mBufferHandle: i32, int32_t resultOffset);
// extern int32_t	mBufferExtendFromSlice(mBufferHandle: i32, int32_t dataOffset, int32_t dataLength);
// extern int32_t	mBufferToBigIntUnsigned(mBufferHandle: i32, bigIntHandle: i32);
// extern int32_t 	mBufferToBigIntSigned(mBufferHandle: i32, bigIntHandle: i32);
// extern int32_t	mBufferFromBigIntUnsigned(mBufferHandle: i32, bigIntHandle: i32);
// extern int32_t	mBufferFromBigIntSigned(mBufferHandle: i32, bigIntHandle: i32);
// extern int32_t	mBufferStorageStore(int32_t keyOffset, int32_t keyLength,mBufferHandle: i32);
// extern int32_t	mBufferStorageLoad(int32_t keyOffset, int32_t keyLength, mBufferHandle: i32);
// extern int32_t	mBufferGetArgument(int32_t id, mBufferHandle: i32);
// extern int32_t	mBufferFinish(mBufferHandle: i32);

#[allow(dead_code)]
extern "C" {
    fn mBufferNew() -> i32;
    fn mBufferNewFromBytes(byte_ptr: *const u8, byte_len: i32) -> i32;
    fn mBufferSetBytes(mBufferHandle: i32, byte_ptr: *const u8, byte_len: i32) -> i32;
    fn mBufferGetLength(mBufferHandle: i32) -> i32;
    fn mBufferGetBytes(mBufferHandle: i32, resultOffset: *mut u8) -> i32;
    fn mBufferExtendFromSlice(mBufferHandle: i32, byte_ptr: *const u8, byte_len: i32) -> i32;
}

impl ManagedBufferApi for crate::ArwenApiImpl {
    fn new_empty(&self) -> Handle {
        unsafe { mBufferNew() }
    }

    fn new_from_bytes(&self, bytes: &[u8]) -> Handle {
        unsafe { mBufferNewFromBytes(bytes.as_ptr(), bytes.len() as i32) }
    }

    fn len(&self, handle: Handle) -> usize {
        unsafe { mBufferGetLength(handle as i32) as usize }
    }

    fn overwrite(&self, handle: Handle, bytes: &[u8]) {
        unsafe {
            let _ = mBufferSetBytes(handle as i32, bytes.as_ptr(), bytes.len() as i32);
        }
    }

    fn extend_from_slice(&self, handle: Handle, bytes: &[u8]) {
        unsafe {
            let _ = mBufferExtendFromSlice(handle as i32, bytes.as_ptr(), bytes.len() as i32);
        }
    }

    fn to_boxed_bytes(&self, handle: Handle) -> BoxedBytes {
        unsafe {
            let len = mBufferGetLength(handle);
            let mut res = BoxedBytes::allocate(len as usize);
            if len > 0 {
                let _ = mBufferGetBytes(handle, res.as_mut_ptr());
            }
            res
        }
    }
}
