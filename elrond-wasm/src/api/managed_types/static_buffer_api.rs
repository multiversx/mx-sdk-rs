use crate::types::LockableStaticBuffer;

pub trait StaticBufferApi {
    type StaticBufferApiImpl: StaticBufferApiImpl;

    fn static_buffer_api_impl() -> Self::StaticBufferApiImpl;
}

/// A raw bytes buffer stored statically:
/// - in wasm as a static variable
/// - in debug mode on the thread local context
pub trait StaticBufferApiImpl {
    fn with_lockable_static_buffer<R, F: FnOnce(&mut LockableStaticBuffer) -> R>(&self, f: F) -> R;
}
