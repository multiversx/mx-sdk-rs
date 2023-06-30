use crate::types::LockableStaticBuffer;

use super::RawHandle;

pub trait StaticVarApi {
    type StaticVarApiImpl: StaticVarApiImpl;

    fn static_var_api_impl() -> Self::StaticVarApiImpl;
}

/// A raw bytes buffer stored statically:
/// - in wasm as a static variable
/// - in debug mode on the thread local context
pub trait StaticVarApiImpl {
    fn with_lockable_static_buffer<R, F: FnOnce(&mut LockableStaticBuffer) -> R>(&self, f: F) -> R;

    fn set_external_view_target_address_handle(&self, handle: RawHandle);

    fn get_external_view_target_address_handle(&self) -> RawHandle;

    fn next_handle(&self) -> RawHandle;

    fn set_num_arguments(&self, num_arguments: i32);

    fn get_num_arguments(&self) -> i32;

    fn set_call_value_egld_handle(&self, handle: RawHandle);

    fn get_call_value_egld_handle(&self) -> RawHandle;

    fn set_call_value_multi_esdt_handle(&self, handle: RawHandle);

    fn get_call_value_multi_esdt_handle(&self) -> RawHandle;
}
