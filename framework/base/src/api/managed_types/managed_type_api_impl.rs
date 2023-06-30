use crate::api::ErrorApi;

use super::{
    token_identifier_util::IDENTIFIER_MAX_LENGTH, BigFloatApiImpl, BigIntApiImpl,
    EllipticCurveApiImpl, ManagedBufferApiImpl, ManagedMapApiImpl,
};

pub trait ManagedTypeApiImpl:
    BigIntApiImpl
    + BigFloatApiImpl
    + EllipticCurveApiImpl
    + ManagedBufferApiImpl
    + ManagedMapApiImpl
    + ErrorApi
{
    fn mb_to_big_int_unsigned(
        &self,
        buffer_handle: Self::ManagedBufferHandle,
        dest: Self::BigIntHandle,
    );

    fn mb_to_big_int_signed(
        &self,
        buffer_handle: Self::ManagedBufferHandle,
        dest: Self::BigIntHandle,
    );

    fn mb_from_big_int_unsigned(
        &self,
        big_int_handle: Self::BigIntHandle,
        dest: Self::ManagedBufferHandle,
    );

    fn mb_from_big_int_signed(
        &self,
        big_int_handle: Self::BigIntHandle,
        dest: Self::ManagedBufferHandle,
    );

    fn mb_to_big_float(&self, buffer_handle: Self::ManagedBufferHandle, dest: Self::BigFloatHandle);

    fn mb_from_big_float(
        &self,
        big_float_handle: Self::BigFloatHandle,
        dest: Self::ManagedBufferHandle,
    );

    fn validate_token_identifier(&self, token_id_handle: Self::ManagedBufferHandle) -> bool {
        let token_id_len = self.mb_len(token_id_handle.clone());
        if token_id_len > IDENTIFIER_MAX_LENGTH {
            return false;
        }

        let mut static_buffer = [0u8; IDENTIFIER_MAX_LENGTH];
        let static_buffer_slice = &mut static_buffer[..token_id_len];

        let load_result = self.mb_load_slice(token_id_handle, 0, static_buffer_slice);
        if load_result.is_err() {
            return false;
        }

        super::token_identifier_util::validate_token_identifier(static_buffer_slice)
    }

    fn get_token_ticker_len(&self, token_id_len: usize) -> usize {
        super::token_identifier_util::get_token_ticker_len(token_id_len)
    }
}
