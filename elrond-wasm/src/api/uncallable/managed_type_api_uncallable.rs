use crate::api::{Handle, ManagedTypeApi, ManagedTypeApiImpl};

use super::UncallableApi;

impl ManagedTypeApi for UncallableApi {
    type ManagedTypeApiImpl = Self;

    fn managed_type_impl() -> Self::ManagedTypeApiImpl {
        unreachable!()
    }
}

impl ManagedTypeApiImpl for UncallableApi {
    fn mb_to_big_int_unsigned(&self, _buffer_handle: Handle) -> Handle {
        unreachable!()
    }

    fn mb_to_big_int_signed(&self, _buffer_handle: Handle) -> Handle {
        unreachable!()
    }

    fn mb_from_big_int_unsigned(&self, _big_int_handle: Handle) -> Handle {
        unreachable!()
    }

    fn mb_from_big_int_signed(&self, _big_int_handle: Handle) -> Handle {
        unreachable!()
    }
}
