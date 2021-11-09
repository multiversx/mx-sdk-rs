use elrond_codec::TryStaticCast;

use crate::api::ErrorApi;

use super::{BigFloatApi, BigIntApi, EllipticCurveApi, ManagedBufferApi};

pub type Handle = i32;

pub trait ManagedTypeApi:
    TryStaticCast
    + BigFloatApi
    + BigIntApi
    + EllipticCurveApi
    + ManagedBufferApi
    + ErrorApi
    + Clone
    + 'static
{
    fn mb_to_big_int_unsigned(&self, buffer_handle: Handle) -> Handle;

    fn mb_to_big_int_signed(&self, buffer_handle: Handle) -> Handle;

    fn mb_from_big_int_unsigned(&self, big_int_handle: Handle) -> Handle;

    fn mb_from_big_int_signed(&self, big_int_handle: Handle) -> Handle;

    fn mb_to_big_float(&self, buffer_handle: Handle) -> Handle;

    fn mb_from_big_float(&self, big_float_handle: Handle) -> Handle;
}
