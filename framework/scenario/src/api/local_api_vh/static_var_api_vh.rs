use crate::api::{VMHooksApi, VMHooksApiBackend};
use multiversx_sc::{
    api::{const_handles, use_raw_handle, RawHandle, StaticVarApi, StaticVarApiImpl},
    types::LockableStaticBuffer,
};

impl<VHB: VMHooksApiBackend> StaticVarApi for VMHooksApi<VHB> {
    type StaticVarApiImpl = Self;

    fn static_var_api_impl() -> Self::StaticVarApiImpl {
        Self::api_impl()
    }
}

impl<VHB: VMHooksApiBackend> StaticVarApiImpl for VMHooksApi<VHB> {
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
                .external_view_target_address_handle = handle;
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
            data.static_vars_cell.borrow_mut().call_value_egld_handle = handle;
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
                .call_value_multi_esdt_handle = handle;
        })
    }

    fn get_call_value_multi_esdt_handle(&self) -> RawHandle {
        self.with_static_data(|data| {
            use_raw_handle(data.static_vars_cell.borrow().call_value_multi_esdt_handle)
        })
    }

    fn set_scaling_factor_init(
        &self,
        scaling_factor: [bool; const_handles::SCALING_FACTOR_LENGTH as usize],
    ) {
        self.with_static_data(|data| {
            data.static_vars_cell.borrow_mut().scaling_factor_init = scaling_factor
        })
    }

    fn get_scaling_factor_init(&self) -> [bool; const_handles::SCALING_FACTOR_LENGTH as usize] {
        self.with_static_data(|data| data.static_vars_cell.borrow().scaling_factor_init)
    }

    // maybe replace with vm hooks call from managed decimal
    fn get_i64_from_handle(&self, handle: RawHandle) -> i64 {
        self.with_vm_hooks(|vm| vm.big_int_get_int64(handle))
    }

    fn set_i64_to_handle(&self, handle: RawHandle, value: i64) {
        self.with_vm_hooks(|vm| {
            vm.big_int_set_int64(handle, value);
        })
    }
}
