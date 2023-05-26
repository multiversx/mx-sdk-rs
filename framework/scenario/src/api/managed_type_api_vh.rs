mod big_float_api_vh;
mod big_int_api_vh;
mod elliptic_curve_api_vh;
mod managed_buffer_api_vh;
mod managed_map_api_vh;
mod static_var_api_vh;

use multiversx_sc::api::{ManagedTypeApi, ManagedTypeApiImpl};

use super::{StaticApi, VMHooksBackend};

impl ManagedTypeApi for StaticApi {
    type ManagedTypeApiImpl = VMHooksBackend;

    fn managed_type_impl() -> Self::ManagedTypeApiImpl {
        VMHooksBackend::static_managed_type_backend()
    }
}

impl ManagedTypeApiImpl for VMHooksBackend {
    fn mb_to_big_int_unsigned(
        &self,
        _buffer_handle: Self::ManagedBufferHandle,
        _dest: Self::BigIntHandle,
    ) {
        unreachable!()
    }

    fn mb_to_big_int_signed(
        &self,
        _buffer_handle: Self::ManagedBufferHandle,
        _dest: Self::BigIntHandle,
    ) {
        unreachable!()
    }

    fn mb_from_big_int_unsigned(
        &self,
        _big_int_handle: Self::BigIntHandle,
        _dest: Self::ManagedBufferHandle,
    ) {
        unreachable!()
    }

    fn mb_from_big_int_signed(
        &self,
        _big_int_handle: Self::BigIntHandle,
        _dest: Self::ManagedBufferHandle,
    ) {
        unreachable!()
    }

    fn mb_to_big_float(
        &self,
        _buffer_handle: Self::ManagedBufferHandle,
        _dest: Self::BigFloatHandle,
    ) {
        unreachable!()
    }

    fn mb_from_big_float(
        &self,
        _big_float_handle: Self::BigFloatHandle,
        _dest: Self::ManagedBufferHandle,
    ) {
        unreachable!()
    }
}
