use crate::types::LockableStaticBuffer;

use super::{RawHandle, StaticVarApiFlags};

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

    fn set_flags(&self, flags: StaticVarApiFlags);

    fn get_flags(&self) -> StaticVarApiFlags;

    /// Returns true if the flag is set, false if is default (false).
    ///
    /// If the flag is unset (false), will set it.
    fn flag_is_set_or_update(&self, flag: StaticVarApiFlags) -> bool {
        let mut current_flags = self.get_flags();
        let contains_flag = current_flags.check_and_set(flag);
        if !contains_flag {
            self.set_flags(current_flags);
        }
        contains_flag
    }

    fn is_scaling_factor_cached(&self, decimals: usize) -> bool;

    fn set_scaling_factor_cached(&self, decimals: usize);
}
