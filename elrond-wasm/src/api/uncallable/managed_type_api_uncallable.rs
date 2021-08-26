use crate::api::{Handle, ManagedTypeApi};

impl ManagedTypeApi for super::UncallableApi {
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
