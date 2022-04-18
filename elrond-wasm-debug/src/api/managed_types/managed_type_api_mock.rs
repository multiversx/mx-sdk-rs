use crate::num_bigint::{BigInt, Sign};
use elrond_wasm::api::{BigIntApi, Handle, ManagedBufferApi, ManagedTypeApi, ManagedTypeApiImpl};

use crate::DebugApi;

impl ManagedTypeApi for DebugApi {
    type ManagedTypeApiImpl = DebugApi;

    fn managed_type_impl() -> Self {
        DebugApi::new_from_static()
    }
}

impl ManagedTypeApiImpl for DebugApi {
    fn mb_to_big_int_unsigned(&self, buffer_handle: Handle, dest: Handle) {
        let bytes = self.mb_to_boxed_bytes(buffer_handle);
        let bi = BigInt::from_bytes_be(Sign::Plus, bytes.as_slice());
        self.bi_overwrite(dest, bi);
    }

    fn mb_to_big_int_signed(&self, buffer_handle: Handle, dest: Handle) {
        let bytes = self.mb_to_boxed_bytes(buffer_handle);
        let bi = BigInt::from_signed_bytes_be(bytes.as_slice());
        self.bi_overwrite(dest, bi);
    }

    fn mb_from_big_int_unsigned(&self, big_int_handle: Handle, dest: Handle) {
        let bi_bytes = self.bi_get_unsigned_bytes(big_int_handle);
        self.mb_overwrite(dest, bi_bytes.as_slice());
    }

    fn mb_from_big_int_signed(&self, big_int_handle: Handle, dest: Handle) {
        let bi_bytes = self.bi_get_signed_bytes(big_int_handle);
        self.mb_overwrite(dest, bi_bytes.as_slice());
    }
}
