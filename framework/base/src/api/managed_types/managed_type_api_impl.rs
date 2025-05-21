use crate::api::ErrorApi;

use super::{
    BigFloatApiImpl, BigIntApiImpl, EllipticCurveApiImpl, ManagedBufferApiImpl, ManagedMapApiImpl,
};

use multiversx_chain_core::token_identifier_util;

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

    fn validate_token_identifier(&self, token_id_handle: Self::ManagedBufferHandle) -> bool;

    fn get_token_ticker_len(&self, token_id_len: usize) -> usize {
        token_identifier_util::get_token_ticker_len(token_id_len)
    }
}
