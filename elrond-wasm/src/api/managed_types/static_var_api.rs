use crate::types::LockableStaticBuffer;

use super::Handle;

pub trait StaticVarApi {
    type StaticVarApiImpl: StaticVarApiImpl;

    const BIG_INT_HANDLE_ZERO: i32 = 0;
    const BIG_INT_HANDLE_START_FROM: i32 = 10; // < 10 reserved for APIs
    const MANAGED_BUFFER_HANDLE_START_FROM: i32 = 10; // < 10 reserved for APIs

    fn static_var_api_impl() -> Self::StaticVarApiImpl;
}

/// A raw bytes buffer stored statically:
/// - in wasm as a static variable
/// - in debug mode on the thread local context
pub trait StaticVarApiImpl {
    fn with_lockable_static_buffer<R, F: FnOnce(&mut LockableStaticBuffer) -> R>(&self, f: F) -> R;

    fn set_external_view_target_address_handle(&self, handle: Handle);

    fn get_external_view_target_address_handle(&self) -> Handle;

    fn next_bigint_handle(&self) -> Handle;

    fn get_next_managed_buffer_handle(&self) -> Handle;
}
