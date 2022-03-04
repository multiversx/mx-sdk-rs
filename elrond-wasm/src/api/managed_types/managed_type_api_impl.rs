use crate::api::ErrorApi;

use super::{
    token_identifier_util::IDENTIFIER_MAX_LENGTH, BigIntApi, EllipticCurveApi, ManagedBufferApi,
};

pub type Handle = i32;

pub trait ManagedTypeApiImpl: BigIntApi + EllipticCurveApi + ManagedBufferApi + ErrorApi {
    fn mb_to_big_int_unsigned(&self, buffer_handle: Handle) -> Handle;

    fn mb_to_big_int_signed(&self, buffer_handle: Handle) -> Handle;

    fn mb_from_big_int_unsigned(&self, big_int_handle: Handle) -> Handle;

    fn mb_from_big_int_signed(&self, big_int_handle: Handle) -> Handle;

    fn validate_token_identifier(&self, token_id_handle: Handle) -> bool {
        let token_id_len = self.mb_len(token_id_handle);
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
}
