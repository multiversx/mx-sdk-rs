use crate::types::LockableStaticBuffer;

use super::{const_handles, RawHandle};

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

    fn set_scaling_factor_init(&self, scaling_factor: [bool; const_handles::SCALING_FACTOR_LENGTH as usize]);

    fn get_scaling_factor_init(&self) -> [bool; const_handles::SCALING_FACTOR_LENGTH as usize];

    fn get_i64_from_handle(&self, handle: RawHandle) -> i64;

    fn set_i64_to_handle(&self, handle: RawHandle, value: i64);
}
