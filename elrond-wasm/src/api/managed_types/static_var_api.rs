use crate::types::LockableStaticBuffer;

use super::Handle;

pub trait StaticVarApi {
    type StaticVarApiImpl: StaticVarApiImpl;

    fn static_var_api_impl() -> Self::StaticVarApiImpl;
}

/// A raw bytes buffer stored statically:
/// - in wasm as a static variable
/// - in debug mode on the thread local context
pub trait StaticVarApiImpl {
    fn with_lockable_static_buffer<R, F: FnOnce(&mut LockableStaticBuffer) -> R>(&self, f: F) -> R;

    fn set_external_view_target_address_handle(&self, handle: Handle);

    fn get_external_view_target_address_handle(&self) -> Handle;
}
