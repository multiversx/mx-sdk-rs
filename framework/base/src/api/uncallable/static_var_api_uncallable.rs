use crate::{
    api::{RawHandle, StaticVarApi, StaticVarApiFlags, StaticVarApiImpl},
    types::LockableStaticBuffer,
};

use super::UncallableApi;

impl StaticVarApi for UncallableApi {
    type StaticVarApiImpl = UncallableApi;

    fn static_var_api_impl() -> Self::StaticVarApiImpl {
        unreachable!()
    }
}

impl StaticVarApiImpl for UncallableApi {
    fn with_lockable_static_buffer<R, F: FnOnce(&mut LockableStaticBuffer) -> R>(
        &self,
        _f: F,
    ) -> R {
        unreachable!()
    }

    fn set_external_view_target_address_handle(&self, _handle: RawHandle) {
        unreachable!()
    }

    fn get_external_view_target_address_handle(&self) -> RawHandle {
        unreachable!()
    }

    fn next_handle(&self) -> RawHandle {
        unreachable!()
    }

    fn set_num_arguments(&self, _num_arguments: i32) {
        unreachable!()
    }

    fn get_num_arguments(&self) -> i32 {
        unreachable!()
    }

    fn set_flags(&self, _flags: StaticVarApiFlags) {
        unreachable!()
    }

    fn get_flags(&self) -> StaticVarApiFlags {
        unreachable!()
    }

    fn is_scaling_factor_cached(&self, _decimals: usize) -> bool {
        unreachable!()
    }

    fn set_scaling_factor_cached(&self, _decimals: usize) {
        unreachable!()
    }
}
