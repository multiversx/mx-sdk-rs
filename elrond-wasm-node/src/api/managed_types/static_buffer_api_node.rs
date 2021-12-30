use elrond_wasm::{
    api::{StaticBufferApi, StaticBufferApiImpl},
    types::LockableStaticBuffer,
};

use crate::VmApiImpl;

static mut STATIC_BUFFER: LockableStaticBuffer = LockableStaticBuffer::new();

impl StaticBufferApi for VmApiImpl {
    type StaticBufferApiImpl = VmApiImpl;

    fn static_buffer_api_impl() -> Self::StaticBufferApiImpl {
        VmApiImpl {}
    }
}

impl StaticBufferApiImpl for VmApiImpl {
    fn with_lockable_static_buffer<R, F: FnOnce(&mut LockableStaticBuffer) -> R>(&self, f: F) -> R {
        unsafe { f(&mut STATIC_BUFFER) }
    }
}
