use elrond_wasm::types::LockableStaticBuffer;

static mut STATIC_BUFFER: LockableStaticBuffer = LockableStaticBuffer::new();

impl elrond_wasm::api::StaticBufferApi for crate::VmApiImpl {
    fn with_lockable_static_buffer<R, F: FnOnce(&mut LockableStaticBuffer) -> R>(&self, f: F) -> R {
        unsafe { f(&mut STATIC_BUFFER) }
    }
}
