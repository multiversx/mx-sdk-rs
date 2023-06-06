mod big_float_api_vh;
mod big_int_api_vh;
mod elliptic_curve_api_vh;
mod managed_buffer_api_vh;
mod managed_map_api_vh;
mod static_var_api_vh;

use multiversx_sc::api::{HandleConstraints, ManagedTypeApi, ManagedTypeApiImpl};

use super::{VMHooksApi, VMHooksBackend, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> ManagedTypeApi for VMHooksApi<BACKEND_TYPE> {
    type ManagedTypeApiImpl = VMHooksBackend;

    fn managed_type_impl() -> Self::ManagedTypeApiImpl {
        Self::backend()
    }
}

impl ManagedTypeApiImpl for VMHooksBackend {
    fn mb_to_big_int_unsigned(
        &self,
        buffer_handle: Self::ManagedBufferHandle,
        big_int_handle: Self::BigIntHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.mbuffer_to_big_int_unsigned(
                buffer_handle.get_raw_handle(),
                big_int_handle.get_raw_handle(),
            )
        });
    }

    fn mb_to_big_int_signed(
        &self,
        buffer_handle: Self::ManagedBufferHandle,
        big_int_handle: Self::BigIntHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.mbuffer_to_big_int_signed(
                buffer_handle.get_raw_handle(),
                big_int_handle.get_raw_handle(),
            )
        });
    }

    fn mb_from_big_int_unsigned(
        &self,
        big_int_handle: Self::BigIntHandle,
        buffer_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.mbuffer_from_big_int_unsigned(
                buffer_handle.get_raw_handle(),
                big_int_handle.get_raw_handle(),
            )
        });
    }

    fn mb_from_big_int_signed(
        &self,
        big_int_handle: Self::BigIntHandle,
        buffer_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.mbuffer_from_big_int_signed(
                buffer_handle.get_raw_handle(),
                big_int_handle.get_raw_handle(),
            )
        });
    }

    fn mb_to_big_float(
        &self,
        buffer_handle: Self::ManagedBufferHandle,
        big_float_handle: Self::BigFloatHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.mbuffer_to_big_float(
                buffer_handle.get_raw_handle(),
                big_float_handle.get_raw_handle(),
            )
        });
    }

    fn mb_from_big_float(
        &self,
        big_float_handle: Self::BigFloatHandle,
        buffer_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.mbuffer_from_big_float(
                buffer_handle.get_raw_handle(),
                big_float_handle.get_raw_handle(),
            )
        });
    }
}
