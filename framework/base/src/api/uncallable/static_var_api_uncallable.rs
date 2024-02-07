use crate::{
    api::{RawHandle, StaticVarApi, StaticVarApiImpl},
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

    fn set_call_value_egld_handle(&self, _handle: RawHandle) {
        unreachable!()
    }

    fn get_call_value_egld_handle(&self) -> RawHandle {
        unreachable!()
    }

    fn set_call_value_multi_esdt_handle(&self, _handle: RawHandle) {
        unreachable!()
    }

    fn get_call_value_multi_esdt_handle(&self) -> RawHandle {
        unreachable!()
    }

    fn set_scaling_factor_init(
        &self,
        _scaling_factor: [bool; crate::api::const_handles::SCALING_FACTOR_LENGTH as usize],
    ) {
        unreachable!()
    }

    fn get_scaling_factor_init(
        &self,
    ) -> [bool; crate::api::const_handles::SCALING_FACTOR_LENGTH as usize] {
        unreachable!()
    }

    fn get_i64_from_handle(&self, _handle: RawHandle) -> i64 {
        unreachable!()
    }

    fn set_i64_to_handle(&self, _handle: RawHandle, _value: i64) {
        unreachable!()
    }
}
