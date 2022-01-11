use crate::DebugApi;
use elrond_wasm::{
    api::{Handle, StaticVarApi, StaticVarApiImpl},
    types::LockableStaticBuffer,
};

impl StaticVarApi for DebugApi {
    type StaticVarApiImpl = DebugApi;

    fn static_var_api_impl() -> DebugApi {
        DebugApi::new_from_static()
    }
}

impl StaticVarApiImpl for DebugApi {
    fn with_lockable_static_buffer<R, F: FnOnce(&mut LockableStaticBuffer) -> R>(&self, f: F) -> R {
        let mut lockable_static_buffer = self.lockable_static_buffer_cell.borrow_mut();
        f(&mut lockable_static_buffer)
    }

    fn set_external_view_target_address_handle(&self, handle: Handle) {
        self.static_vars_cell
            .borrow_mut()
            .external_view_target_address_handle = handle;
    }

    fn get_external_view_target_address_handle(&self) -> Handle {
        self.static_vars_cell
            .borrow()
            .external_view_target_address_handle
    }
}
