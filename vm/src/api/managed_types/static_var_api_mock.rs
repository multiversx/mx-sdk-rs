use crate::DebugApi;
use multiversx_sc::{
    api::{use_raw_handle, HandleConstraints, StaticVarApi, StaticVarApiImpl},
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

    fn set_external_view_target_address_handle(&self, handle: Self::ManagedBufferHandle) {
        self.static_vars_cell
            .borrow_mut()
            .external_view_target_address_handle = handle.get_raw_handle();
    }

    fn get_external_view_target_address_handle(&self) -> Self::ManagedBufferHandle {
        use_raw_handle(
            self.static_vars_cell
                .borrow()
                .external_view_target_address_handle,
        )
    }

    fn next_handle<H: HandleConstraints>(&self) -> H {
        let mut ref_tx_static_vars = self.static_vars_cell.borrow_mut();
        let new_handle = ref_tx_static_vars.next_handle;
        ref_tx_static_vars.next_handle -= 1;
        use_raw_handle(new_handle)
    }

    fn set_num_arguments(&self, num_arguments: i32) {
        self.static_vars_cell.borrow_mut().num_arguments = num_arguments;
    }

    fn get_num_arguments(&self) -> i32 {
        self.static_vars_cell.borrow().num_arguments
    }

    fn set_call_value_egld_handle(&self, handle: Self::BigIntHandle) {
        self.static_vars_cell.borrow_mut().call_value_egld_handle = handle.get_raw_handle();
    }

    fn get_call_value_egld_handle(&self) -> Self::BigIntHandle {
        use_raw_handle(self.static_vars_cell.borrow().call_value_egld_handle)
    }

    fn set_call_value_multi_esdt_handle(&self, handle: Self::ManagedBufferHandle) {
        self.static_vars_cell
            .borrow_mut()
            .call_value_multi_esdt_handle = handle.get_raw_handle();
    }

    fn get_call_value_multi_esdt_handle(&self) -> Self::ManagedBufferHandle {
        use_raw_handle(self.static_vars_cell.borrow().call_value_multi_esdt_handle)
    }
}
