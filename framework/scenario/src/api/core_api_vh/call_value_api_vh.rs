use multiversx_sc::api::{CallValueApi, CallValueApiImpl};

use crate::api::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> CallValueApi for VMHooksApi<BACKEND_TYPE> {
    type CallValueApiImpl = Self;

    fn call_value_api_impl() -> Self::CallValueApiImpl {
        Self::api_impl()
    }
}

impl<const BACKEND_TYPE: VMHooksBackendType> CallValueApiImpl for VMHooksApi<BACKEND_TYPE> {
    fn check_not_payable(&self) {
        self.with_vm_hooks(|vh| vh.check_no_payment())
    }

    fn load_egld_value(&self, dest: Self::BigIntHandle) {
        self.with_vm_hooks(|vh| vh.big_int_get_call_value(dest));
    }

    fn load_all_esdt_transfers(&self, dest_handle: Self::ManagedBufferHandle) {
        self.with_vm_hooks(|vh| vh.managed_get_multi_esdt_call_value(dest_handle));
    }

    fn esdt_num_transfers(&self) -> usize {
        self.with_vm_hooks(|vh| vh.get_num_esdt_transfers()) as usize
    }
}
