use crate::api::ErrorApi;

use super::{BigIntApi, EllipticCurveApi, ManagedBufferApi};

pub type Handle = i32;

pub trait ManagedTypeApiImpl: BigIntApi + EllipticCurveApi + ManagedBufferApi + ErrorApi {
    fn mb_to_big_int_unsigned(&self, buffer_handle: Handle) -> Handle;

    fn mb_to_big_int_signed(&self, buffer_handle: Handle) -> Handle;

    fn mb_from_big_int_unsigned(&self, big_int_handle: Handle) -> Handle;

    fn mb_from_big_int_signed(&self, big_int_handle: Handle) -> Handle;

    fn validate_token_identifier(&self, token_id_handle: Handle) -> bool {
        let token_id_bytes = self.mb_to_boxed_bytes(token_id_handle);
        super::token_identifier_util::validate_token_identifier(token_id_bytes.as_slice())
    }
}
