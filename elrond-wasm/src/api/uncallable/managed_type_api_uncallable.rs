use crate::api::{ManagedTypeApi, ManagedTypeApiImpl};

use super::UncallableApi;

impl ManagedTypeApi for UncallableApi {
    type ManagedTypeApiImpl = Self;

    fn managed_type_impl() -> Self::ManagedTypeApiImpl {
        unreachable!()
    }
}

impl ManagedTypeApiImpl for UncallableApi {
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
