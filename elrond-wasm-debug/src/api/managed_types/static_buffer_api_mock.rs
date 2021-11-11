use elrond_wasm::{api::StaticBufferApi, types::LockableStaticBuffer};
use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::DebugApi;

lazy_static! {
    static ref BUFFER_MUTEX: Mutex<LockableStaticBuffer> = Mutex::new(LockableStaticBuffer::new());
}

impl StaticBufferApi for DebugApi {
    fn with_lockable_static_buffer<R, F: FnOnce(&mut LockableStaticBuffer) -> R>(f: F) -> R {
        f(&mut *BUFFER_MUTEX.lock().unwrap())
    }
}
