use multiversx_sc::api::{EndpointArgumentApi, EndpointArgumentApiImpl};

use super::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> EndpointArgumentApi for VMHooksApi<BACKEND_TYPE> {
    type EndpointArgumentApiImpl = Self;

    fn argument_api_impl() -> Self::EndpointArgumentApiImpl {
        Self::api_impl()
    }
}

impl<const BACKEND_TYPE: VMHooksBackendType> EndpointArgumentApiImpl for VMHooksApi<BACKEND_TYPE> {
    fn get_num_arguments(&self) -> i32 {
        self.with_vm_hooks(|vh| vh.get_num_arguments())
    }

    fn load_argument_managed_buffer(&self, arg_id: i32, dest: Self::ManagedBufferHandle) {
        self.with_vm_hooks(|vh| vh.mbuffer_get_argument(arg_id, dest));
    }

    fn load_callback_closure_buffer(&self, _dest: Self::ManagedBufferHandle) {
        todo!()
    }

    fn get_argument_u64(&self, arg_index: i32) -> u64 {
        self.with_vm_hooks(|vh| vh.small_int_get_unsigned_argument(arg_index)) as u64
    }

    fn get_argument_i64(&self, arg_index: i32) -> i64 {
        self.with_vm_hooks(|vh| vh.small_int_get_signed_argument(arg_index))
    }
}
