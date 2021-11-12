use crate::DebugApi;
use elrond_wasm::{api::StaticBufferApi, types::LockableStaticBuffer};

impl StaticBufferApi for DebugApi {
    fn with_lockable_static_buffer<R, F: FnOnce(&mut LockableStaticBuffer) -> R>(&self, f: F) -> R {
        let mut managed_types = self.m_types_borrow_mut();
        f(&mut managed_types.lockable_static_buffer)
    }
}
