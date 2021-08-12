use elrond_wasm::api::{Handle, ManagedTypeApi};

use crate::TxContext;

impl ManagedTypeApi for TxContext {
    fn managed_buffer_to_big_int_signed(&self, _buffer_handle: Handle) -> Handle {
        unreachable!()
    }

    fn big_int_to_managed_buffer_signed(&self, _big_int_handle: Handle) -> Handle {
        unreachable!()
    }
}
