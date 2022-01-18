use crate::{
    api::{Handle, StaticVarApi, StaticVarApiImpl},
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

    fn set_external_view_target_address_handle(&self, _handle: Handle) {
        unreachable!()
    }

    fn get_external_view_target_address_handle(&self) -> Handle {
        unreachable!()
    }
}
