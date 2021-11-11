use elrond_codec::TryStaticCast;

use crate::api::ErrorApi;

use super::{BigIntApi, EllipticCurveApi, ManagedBufferApi, StaticBufferApi};

pub type Handle = i32;

pub trait ManagedTypeApi:
    TryStaticCast
    + BigIntApi
    + EllipticCurveApi
    + ManagedBufferApi
    + StaticBufferApi
    + ErrorApi
    + Clone
    + 'static
{
    fn mb_to_big_int_unsigned(&self, buffer_handle: Handle) -> Handle;

    fn mb_to_big_int_signed(&self, buffer_handle: Handle) -> Handle;

    fn mb_from_big_int_unsigned(&self, big_int_handle: Handle) -> Handle;

    fn mb_from_big_int_signed(&self, big_int_handle: Handle) -> Handle;
}
