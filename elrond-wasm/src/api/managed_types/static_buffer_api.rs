use crate::types::LockableStaticBuffer;

/// A raw bytes buffer managed by Arwen.
pub trait StaticBufferApi {
    fn with_lockable_static_buffer<R, F: FnOnce(&mut LockableStaticBuffer) -> R>(&self, f: F) -> R;
}
