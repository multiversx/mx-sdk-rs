use crate::api::{Handle, ManagedTypeApi};

impl ManagedTypeApi for super::UncallableApi {
    fn managed_buffer_to_big_int_signed(&self, _buffer_handle: Handle) -> Handle {
        unreachable!()
    }
}
