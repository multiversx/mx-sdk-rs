use alloc::string::String;
use elrond_wasm::api::ManagedBufferApi;
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
    fn mBufferToBigIntUnsigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
    fn mBufferToBigIntSigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
    fn mBufferFromBigIntUnsigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
    fn mBufferFromBigIntSigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
    fn mBufferStorageStore(keyOffset: *const u8, keyLength: i32, mBufferHandle: i32) -> i32;
    fn mBufferStorageLoad(keyOffset: *const u8, keyLength: i32, mBufferHandle: i32) -> i32;
    fn mBufferGetArgument(argId: i32, mBufferHandle: i32) -> i32;
    fn mBufferFinish(mBufferHandle: i32) -> i32;
}

pub struct ArwenManagedBuffer {
    pub handle: i32, // TODO: fix visibility
}

impl elrond_wasm::abi::TypeAbi for ArwenManagedBuffer {
    fn type_name() -> String {
        String::from("bytes")
    }
}

/// A raw bytes buffer managed by Arwen.
impl ManagedBufferApi for ArwenManagedBuffer {
    fn new_empty() -> Self {
        unsafe {
            ArwenManagedBuffer {
                handle: mBufferNew(),
            }
        }
    }

    fn new_from_bytes(bytes: &[u8]) -> Self {
        unsafe {
            ArwenManagedBuffer {
                handle: mBufferNewFromBytes(bytes.as_ptr(), bytes.len() as i32),
            }
        }
    }

    fn len(&self) -> usize {
        unsafe { mBufferGetLength(self.handle as i32) as usize }
    }

    fn overwrite(&mut self, bytes: &[u8]) {
        unsafe {
            mBufferSetBytes(self.handle as i32, bytes.as_ptr(), bytes.len() as i32);
        }
    }

    fn extend_from_slice(&mut self, bytes: &[u8]) {
        unsafe {
            mBufferExtendFromSlice(self.handle as i32, bytes.as_ptr(), bytes.len() as i32);
        }
    }

    fn to_boxed_bytes(&self) -> BoxedBytes {
        panic!()
    }
}
