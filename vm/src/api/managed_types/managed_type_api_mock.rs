use std::convert::TryInto;

use crate::num_bigint::{BigInt, Sign};
use multiversx_sc::api::{BigIntApi, ManagedBufferApi, ManagedTypeApi, ManagedTypeApiImpl};

use crate::DebugApi;

impl ManagedTypeApi for DebugApi {
    type ManagedTypeApiImpl = DebugApi;

    fn managed_type_impl() -> Self {
        DebugApi::new_from_static()
    }
}

impl ManagedTypeApiImpl for DebugApi {
    fn mb_to_big_int_unsigned(
        &self,
        buffer_handle: Self::ManagedBufferHandle,
        dest: Self::BigIntHandle,
    ) {
        let bytes = self.mb_to_boxed_bytes(buffer_handle);
        let bi = BigInt::from_bytes_be(Sign::Plus, bytes.as_slice());
        self.bi_overwrite(dest, bi);
    }

    fn mb_to_big_int_signed(
        &self,
        buffer_handle: Self::ManagedBufferHandle,
        dest: Self::BigIntHandle,
    ) {
        let bytes = self.mb_to_boxed_bytes(buffer_handle);
        let bi = BigInt::from_signed_bytes_be(bytes.as_slice());
        self.bi_overwrite(dest, bi);
    }

    fn mb_from_big_int_unsigned(
        &self,
        big_int_handle: Self::BigIntHandle,
        dest: Self::ManagedBufferHandle,
    ) {
        let bi_bytes = self.bi_get_unsigned_bytes(big_int_handle);
        self.mb_overwrite(dest, bi_bytes.as_slice());
    }

    fn mb_from_big_int_signed(
        &self,
        big_int_handle: Self::BigIntHandle,
        dest: Self::ManagedBufferHandle,
    ) {
        let bi_bytes = self.bi_get_signed_bytes(big_int_handle);
        self.mb_overwrite(dest, bi_bytes.as_slice());
    }

    fn mb_to_big_float(
        &self,
        buffer_handle: Self::ManagedBufferHandle,
        dest: Self::BigFloatHandle,
    ) {
        let mb_bytes = self.mb_get(buffer_handle);
        let float_bytes: [u8; 8] = mb_bytes
            .as_slice()
            .try_into()
            .expect("slice with incorrect length");
        let new_bf = f64::from_be_bytes(float_bytes);
        self.bf_overwrite(dest, new_bf);
    }

    fn mb_from_big_float(
        &self,
        big_float_handle: Self::BigFloatHandle,
        dest: Self::ManagedBufferHandle,
    ) {
        let bf = self.bf_get_f64(big_float_handle);
        let bf_bytes = bf.to_be_bytes();
        self.mb_overwrite(dest, &bf_bytes[..]);
    }
}
