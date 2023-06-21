use crate::api::{VMHooksApi, VMHooksBackendType};
use multiversx_sc::{
    api::{use_raw_handle, HandleConstraints, RawHandle, StaticVarApi, StaticVarApiImpl},
    types::LockableStaticBuffer,
};

impl<const BACKEND_TYPE: VMHooksBackendType> StaticVarApi for VMHooksApi<BACKEND_TYPE> {
    type StaticVarApiImpl = Self;

    fn static_var_api_impl() -> Self::StaticVarApiImpl {
        Self
    }
}

impl<const BACKEND_TYPE: VMHooksBackendType> StaticVarApiImpl for VMHooksApi<BACKEND_TYPE> {
    fn with_lockable_static_buffer<R, F: FnOnce(&mut LockableStaticBuffer) -> R>(&self, f: F) -> R {
        self.with_static_data(|data| {
            let mut lockable_static_buffer = data.lockable_static_buffer_cell.borrow_mut();
            f(&mut lockable_static_buffer)
        })
    }

    fn set_external_view_target_address_handle(&self, handle: RawHandle) {
        self.with_static_data(|data| {
            data.static_vars_cell
                .borrow_mut()
                .external_view_target_address_handle = handle.get_raw_handle();
        });
    }

    fn get_external_view_target_address_handle(&self) -> RawHandle {
        self.with_static_data(|data| {
            data.static_vars_cell
                .borrow()
                .external_view_target_address_handle
        })
    }

    fn next_handle(&self) -> RawHandle {
        self.with_static_data(|data| {
            let mut ref_tx_static_vars = data.static_vars_cell.borrow_mut();
            let new_handle = ref_tx_static_vars.next_handle;
            ref_tx_static_vars.next_handle -= 1;
            new_handle
        })
    }

    fn set_num_arguments(&self, num_arguments: i32) {
        self.with_static_data(|data| {
            data.static_vars_cell.borrow_mut().num_arguments = num_arguments;
        })
    }

    fn get_num_arguments(&self) -> i32 {
        self.with_static_data(|data| data.static_vars_cell.borrow().num_arguments)
    }

    fn set_call_value_egld_handle(&self, handle: RawHandle) {
        self.with_static_data(|data| {
            data.static_vars_cell.borrow_mut().call_value_egld_handle = handle.get_raw_handle();
        })
    }

    fn get_call_value_egld_handle(&self) -> RawHandle {
        self.with_static_data(|data| {
            use_raw_handle(data.static_vars_cell.borrow().call_value_egld_handle)
        })
    }

    fn set_call_value_multi_esdt_handle(&self, handle: RawHandle) {
        self.with_static_data(|data| {
            data.static_vars_cell
                .borrow_mut()
                .call_value_multi_esdt_handle = handle.get_raw_handle();
        })
    }

    fn get_call_value_multi_esdt_handle(&self) -> RawHandle {
        self.with_static_data(|data| {
            use_raw_handle(data.static_vars_cell.borrow().call_value_multi_esdt_handle)
        })
    }
}
